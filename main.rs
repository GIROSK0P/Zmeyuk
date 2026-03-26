use String;
use crossterm;
use crossterm::execute;
use crossterm::terminal::{Clear,ClearType};
use crossterm::event::KeyCode;
use std::time::Duration;
use std::io;
use std::thread;
use rand::Rng;
fn otris(field: Vec<Vec<&str>>){
    let _ = execute!(std::io::stdout(), Clear(ClearType::All), crossterm::cursor::MoveTo(0,0));
    for f_line in &field{
        let mut pr_line=String::new();
        for f_one in f_line{
            pr_line.push_str(f_one);
        }
        println!("{}",pr_line);
    }
}
fn main()  -> io::Result<()> {
    const XS: usize = 18;
    const YS: usize = 18;
    let mut rng = rand::thread_rng();
    let field: Vec<Vec<&str>> = vec![vec!["🟩";XS];YS];
    let cord_size: usize = 2;
    let mut apple_cord: Vec<usize>=vec![5,5]; 
    let mut zmeyuk = vec![vec![1;cord_size]];
    let field_over= vec![vec!["🟥";XS];YS];
    let mut motion_save = 'd';
    let mut sch = 0;
    let _ = crossterm::terminal::enable_raw_mode();
    loop{

    
    if crossterm::event::poll(Duration::from_millis(10))?{
        let motion = crossterm::event::read().unwrap();
        match motion{
            crossterm::event::Event::Key(key_ev) => {
                match key_ev.code{
                    KeyCode::Char(c) =>{motion_save=c;},
                    _ => {},
                }
            },
            crossterm::event::Event::Mouse(_) => {},
            crossterm::event::Event::Resize(..) => {},
            _ => println!("Пу Пу Пу"),
        }
    }
    let mut field_zm = field.clone();
    field_zm[apple_cord[1]][apple_cord[0]]="🆘";
    for zm in &zmeyuk{
        field_zm[zm[1]][zm[0]] = "🟦";
    }
    field_zm[zmeyuk[0][1]][zmeyuk[0][0]]="🧿";
    field_zm[0]=vec!["⬛";XS];
    field_zm[17]=vec!["⬛";XS];
    for i in 0..field_zm.len()-1{
        field_zm[i][0]="⬛";
        field_zm[i][17]="⬛";
    }
    otris(field_zm);
    
    if sch%20==0{
        match motion_save{
            'w' => {
                if zmeyuk.contains(&vec![zmeyuk[0][0],zmeyuk[0][1]-1]) || zmeyuk[0][1]-1==17 ||  zmeyuk[0][1]-1==0 {
                    otris(field_over);
                    println!("ВЫ ПРОИГРАЛИ"); 
                    break;}
                else {
                    zmeyuk.insert(0,vec![zmeyuk[0][0],zmeyuk[0][1]-1])}},
            'a' => {
                if zmeyuk.contains(&vec![zmeyuk[0][0]-1,zmeyuk[0][1]]) || zmeyuk[0][0]-1==17 ||  zmeyuk[0][0]-1==0{
                    otris(field_over);
                    println!("ВЫ ПРОИГРАЛИ"); 
                    break;}
                else {
                    zmeyuk.insert(0,vec![zmeyuk[0][0]-1,zmeyuk[0][1]])}},
            's' => {
                if zmeyuk.contains(&vec![zmeyuk[0][0],zmeyuk[0][1]+1]) || zmeyuk[0][1]+1==17 ||  zmeyuk[0][1]+1==0{
                    otris(field_over);
                    println!("ВЫ ПРОИГРАЛИ"); 
                    break;}
                else {
                    zmeyuk.insert(0,vec![zmeyuk[0][0],zmeyuk[0][1]+1])}},
            'd' => {
                if zmeyuk.contains(&vec![zmeyuk[0][0]+1,zmeyuk[0][1]])|| zmeyuk[0][0]+1==17 ||  zmeyuk[0][0]+1==0{
                    otris(field_over);
                    println!("ВЫ ПРОИГРАЛИ"); 
                    break;}
                else {
                    zmeyuk.insert(0,vec![zmeyuk[0][0]+1,zmeyuk[0][1]])}},
            _ => println!("КАК ТАК ТО"),
        }

        if zmeyuk[0]==apple_cord{
            
            while zmeyuk.contains(&apple_cord){
                let mut apple_cord_new: usize=rng.gen_range(1..=15);
                apple_cord[0]=apple_cord_new.clone();
                apple_cord_new=rng.gen_range(1..=15);
                apple_cord[1]=apple_cord_new.clone();
            }
        } else {
            zmeyuk.pop();
    }
    }
    
    
    sch+=1;
    thread::sleep(Duration::from_millis(2));
}
    Ok(())
}
