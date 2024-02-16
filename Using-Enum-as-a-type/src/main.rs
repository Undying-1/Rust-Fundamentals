
#[derive(Debug)]
enum WineRegions {
    Bordeaux,
    Burgundy,
    Champagne,
    Tuscany,
    Rioja,
    NapaValley,
}


#[derive(Debug)]
enum Popularity{
    High,
    Middle,
    Low,
}

struct Wine {
    name: String,
    region: WineRegions,
}


fn supported_regions(w: &WineRegions){
    match w {
        WineRegions::Rioja => println!("Rioja is supported"),
        _ => println!("{:?} is not supported", w),
    }
}


fn region_popularity(w: &WineRegions){
    match w {
        WineRegions::Bordeaux => println!("{:?} Popular", Popularity::High),
        WineRegions::Champagne => println!("{:?} Popular", Popularity::Middle),
        WineRegions::Rioja => println!("{:?} Popular", Popularity::Low),
        _ => println!("Unknown Popular"),
    }
}




fn main() {
    let wine1 = Wine {
        name: String::from("Chateau Margaux"),
        region: WineRegions::Bordeaux,
    };

    let wine2 = Wine {
        name: String::from("Barolo"),
        region: WineRegions::Tuscany,
    };

    println!("Wine 1: {} form {:?}", wine1.name, wine1.region);
    println!("Wine 2: {} fron {:?}", wine2.name, wine2.region);

    supported_regions(&wine1.region);
    supported_regions(&WineRegions::Rioja);

    region_popularity(&wine1.region);
    region_popularity(&wine2.region);
    region_popularity(&WineRegions::Rioja);
}
