use ::chrono::{Duration, DateTime, Utc};
use chrono::prelude::*;

pub fn mean(stats: &Vec<u8>) -> f32 {
    // Returns mean, DUR!!!

    let n = stats.len();
    if n==0 { return 0. }
    let n = n as f32;
    let sum : f32 = stats.iter().map(|x| *x as f32).sum();
    sum/n
}

pub fn sigma(stats: &Vec<u8>, mean: f32) -> f32 {
    // Mean is passed in to prevent duplicate computation
    let n = stats.len();
    if n==0 { return 0. }
    let n = n as f32;
    let ss : f32 = stats.iter().map(|x| (*x as f32 - mean).powf(2.)).sum();
    let result = (ss/(n-1.)).powf(0.5);
    if result.is_nan() { 0. } else { result }
}


pub fn date_from_msl(mins: u32) -> DateTime<Utc> {
    Utc.ymd(2015, 5, 1).and_hms(20, 0, 0) + Duration::minutes(mins as i64)
}

pub fn upper_month_from_msl(mins: u32) -> DateTime<Utc> {
    let date = date_from_msl(mins);
    let month = date.month();
    let year = date.year();
    Utc.ymd(year, month, 1).and_hms(0,0,0)
}

pub fn x_grouper(ys: &Vec<u8>, xs: &Vec<u32>, minutes: u32) -> [Vec<f32>;2] {
    let n_points = ys.len();
    let mut y_sums : Vec<f32> = vec![];
    let mut y_counts : Vec<f32> = vec![];

    let mut upper_x = xs[n_points-1];
    let mut upper_month = upper_month_from_msl(upper_x);
    println!("Minutes {}",minutes);
    let mut y_sum : f32 = 0.;
    let mut y_count : f32 = 0.;
    for p in 0..n_points {
        let point = n_points-p-1;
        let x = xs[point];
        let condition = if minutes > 0 { x < upper_x - minutes } else { date_from_msl(x) < upper_month };
        if condition {
            y_sums.push(y_sum);
            y_counts.push(y_count);
            y_sum = 0.;
            y_count = 0.;
            if minutes > 0 { 
                upper_x = upper_x - minutes 
            }
            else { upper_month = upper_month_from_msl(x) }
        }
        y_sum += ys[point] as f32;
        y_count += 1.;
    }
    y_sums.push(y_sum);
    y_counts.push(y_count);
    println!("ysums: {:?}, ycounts: {:?}",y_sums, y_counts);
    [y_sums,y_counts]
}

pub fn exponential_smoother(ys: &Vec<u8>, xs: &Vec<u32>, threshold: f32) -> [Vec<f32>;2] {
    let filter_outliers = false;
    let should_smooth = false;

    let n_points = ys.len();
    let mut smoothed_y : Vec<f32> = vec![];
    if should_smooth {
        let alpha_m1 : f32 = 0.99;
        let alpha_cutoff = 458; // round(log(0.01)/(log(1-ALPHA)));
        let mut exp_den : [f32;458] = [1.0;458];
        let mut exp_vals : [f32;458] = [1.0;458];
        for i in 1..alpha_cutoff {
            let new_weight : f32 = exp_vals[i-1]*alpha_m1;
            exp_vals[i] = new_weight;
            exp_den[i] = exp_den[i-1]+new_weight;
        }

        for x in 1..n_points {
            let mut num = ys[x] as f32;
            let den = exp_den[if alpha_cutoff < x { alpha_cutoff } else { x }];
            let mut expo = 1;
            for z in 0..x-1 {
                let y = x - z;
                num += (ys[y] as f32)*exp_vals[expo];
                expo += 1;
                if expo >= alpha_cutoff {
                    break;
                } else if x==1 {
                    continue;
                }
                let res = num/den;
                smoothed_y.push(res);
            }
        }
    } else {
        let mut sum_around = 0.;
        let mut sum_count = 0.;
        let points_around = 5;
        for x in 0..n_points {
            let min = if x < points_around { 0 } else { x-points_around };
            let max = if x+points_around >= n_points { n_points - 1 } else { x+points_around - 1 };
            for i in min..max {
                sum_around += ys[i+1] as f32;
                sum_count += 1.;
            }
            smoothed_y.push( sum_around/sum_count );
            sum_around = 0.;
            sum_count = 0.;
        }
    }

    let mut smoothed_x : Vec<f32> = vec![];
    let mut counter = 0;
    let offset = if should_smooth { 2 } else { 0 };

    if filter_outliers {
        let ny : f32 = smoothed_y.len() as f32;
        let mean_y : f32 = smoothed_y.iter().fold(0., |sum, val| sum + val)/ny;
        let variance_times_n : f32 = smoothed_y.iter().map(|y| (y-mean_y).powf(2.)).sum();
        let sigma_y : f32 = (variance_times_n/ny).powf(0.5);
        for p in 0..n_points-offset {
            let y = smoothed_y[p];
            if (y-mean_y).abs() > 2.*sigma_y { continue; } // outlier
            smoothed_y[counter] = y;
            smoothed_x.push(xs[p] as f32);
            counter += 1;
        }
        smoothed_y = smoothed_y[0..counter].to_vec();
    } else {
        for p in 0..n_points-offset { smoothed_x.push(xs[p] as f32); }
        counter = smoothed_y.len();
    }

    let mut r = 0;
    if counter as f32 > threshold {
        let bin_size = (counter as f32)/threshold;
        let mut p=0;
        let mut c=0;
        let mut totx = 0.;
        let mut toty = 0.;
        let mut prevx = xs[0] as f32;
        while p < counter {
            let x = smoothed_x[p];
            let y = smoothed_y[p];
            if (p as f32)/bin_size > r as f32 && x - prevx > 0.001 {
                prevx = x;
                smoothed_x[r] = totx/(c as f32);
                smoothed_y[r] = toty/(c as f32);
                r += 1;
                totx = 0.;
                toty = 0.;
                c = 0;
            }
            totx += x;
            toty += y;
            p += 1;
            c += 1;
        }
        smoothed_x = smoothed_x[0..r].to_vec();
        smoothed_y = smoothed_y[0..r].to_vec();
    }
    [smoothed_x,smoothed_y]

}