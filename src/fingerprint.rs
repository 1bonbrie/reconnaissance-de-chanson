use rustfft::{FftPlanner, num_complex::Complex};

const TAILLE_TRAME: usize = 1024;


pub fn transformation_fourier(fenetre: &[f32]) -> Vec<Complex<f32>> {

    let mut buffer = Vec::new();
    for &echantillon in fenetre {
        buffer.push(Complex { re: echantillon, im: 0.0 });
    }

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(TAILLE_TRAME);

    fft.process(&mut buffer);    
    buffer
}

// Retourne le spectrogramme séparé en trames de taille 1024
pub fn generer_spectrogramme(mut signal: Vec<f32>) -> Vec<Vec<Complex<f32>>> {

    while signal.len() % taille != 0 {
        signal.push(0.0);
    }

    let mut spectrogramme = Vec::new();
    let mut i = 0;

    while i + TAILLE_TRAME <= signal.len() {
        let segment = &signal[i.. i + TAILLE_TRAME];
        let fréquences = transformation_fourier(segment);
        spectrogramme.push(fréquences);
        i += TAILLE_TRAME;
    }

    spectrogramme 
}

// Trouve les pics de fréquence de chaque trame du spectrogramme
pub fn trouver_pics(spectrogramme: &[Vec<Complex<f32>>], _nombre_pics: usize) -> Vec<(usize, usize, f32)> {
    let mut pics = Vec::new();

    for temps in 0..spectrogramme.len() {
        let colonnes = &spectrogramme[temps];

        let mut freq_max = 0;
        let mut amp_max = 0.0;

        for (f, c) in colonnes.iter().enumerate() {
            let amp = c.norm();
            if amp > amp_max {
                amp_max = amp;
                freq_max = f;
            }
        }

        pics.push((temps, freq_max));
    }

    pics
}

// Génère les empreintes à partir des pics
pub fn generer_empreintes(
    pics: &[(usize, usize)], 
    _max_delta_temps: usize,      
) -> Vec<(usize, usize, usize)> {
    let mut empreintes = Vec::new();
    
    for i in 0..(pics.len() - 1) {
        let (temps1, freq1) = pics[i];
        let (temps2, freq2) = pics[i + 1];
        
        let delta_temps = temps2 - temps1;
        empreintes.push((freq1, freq2, delta_temps));
    }

    empreintes
}
