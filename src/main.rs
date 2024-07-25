use rand::Rng;

static QUANTIZED_DOMAIN_MAX: usize = 11;

fn main() {
    //let signal: Vec<f64> = gen_random_signal(100)
    //    .expect("Failed to generate random signal"); 
    
    let signal: Vec<f64> = gen_sine_signal(100, 1.0, 1.57) 
        .expect("Failed to generate sine wave signal");


    let sample_size: usize = signal.len();

    if sample_size == 0 {
        eprintln!("Sample size found as a non usized value");
    }


    let mean: f64 = get_mean(signal.clone(), sample_size.clone())
        .expect("Failed to get mean of signal");

    let std_dev: f64 = get_std_dev(signal.clone(), mean, sample_size.clone())
        .expect("Failed to get standard deviation of signal");

    let rms: f64 = get_rms(signal.clone(), sample_size.clone())
        .expect("Failed to get root-mean-square of signal");

    let err: f64 = get_typical_error(std_dev.clone(), sample_size.clone())
        .expect("Failed to get typical error of signal");

    let cov: f64 = get_cov(mean.clone(), std_dev.clone())
        .expect("Failed to get coefficient of variation of signal");

    
    println!("---------------------------");
    println!("Signal Probability Analysis");
    println!("---------------------------");

    println!("mean: {:.3}", mean);
    println!("standard deviation: {:.3}", std_dev);
    println!("root-mean-square: {:.3}V", rms);
    println!("typical error: {:.3}", err);
    println!("coefficient of variation: {:.3}%", cov);
}

fn gen_sine_signal(sample_size: usize, amplitude: f64, period: f64) -> Result<Vec<f64>, ()> {
    let mut signal: Vec<f64> = Vec::new();

    for x in 0..=sample_size {
        signal.push(amplitude * (period * x as f64).sin());
    }

    return Ok(signal);
}

fn gen_random_signal(sample_size: usize) -> Result<Vec<f64>, ()> {
    let mut signal: Vec<f64> = Vec::new();

    for _ in 0..=sample_size {
        signal.push(rand::thread_rng().gen_range(0.0..=10.0));
    }

    return Ok(signal);
}

fn get_mean(signal: Vec<f64>, sample_size: usize) -> Result<f64, ()> {
    let mut sum: f64 = 0.0;

    // summing all values in signal from 0 to max number of possible values
    // from -1.0 to 1.0 (which is 21)
    for sample in 0..QUANTIZED_DOMAIN_MAX {
        sum = sum + (sample as f64) * signal[sample];
    }

    let mean: f64 = sum / sample_size as f64;

    return Ok(mean);
}

fn get_std_dev(signal: Vec<f64>, mean: f64, sample_size: usize) -> Result<f64, ()> {
    let mut sum: f64 = 0.0;

    for sample in 0..QUANTIZED_DOMAIN_MAX {
        sum = ((sample as f64) - mean).sqrt() * signal[sample];
    }

    let std_dev: f64 = sum / ((sample_size as f64) - 1.0);

    return Ok(std_dev);
}

fn get_rms(signal: Vec<f64>, sample_size: usize) -> Result<f64, ()> {
    let mut sum: f64 = 0.0;
    
    for sample in 0..QUANTIZED_DOMAIN_MAX {
        sum += (signal[sample]).powf(2.0);
    }

    let rms = (sum / sample_size as f64).sqrt();
    Ok(rms)
}

fn get_typical_error(std_dev: f64, sample_size: usize) -> Result<f64, ()> {
    return Ok(std_dev/(sample_size.pow(2) as f64));
}

fn get_cov(mean: f64, std_dev: f64) -> Result<f64, ()> {
    return Ok((std_dev/mean) * 100.0);
}
