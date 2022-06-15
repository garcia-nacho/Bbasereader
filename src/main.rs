/* 
BamBaseReader
Licenced under GPL v2 licence
Ignacio Garcia Llorente 2021 
iggl@fhi.no
*/


use rust_htslib::{bam, bam::Read, bam::IndexedReader};
use clap::{Arg, App};
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::iter;
use std::str;
use regex::Regex;
use std::fs;


fn main() {
    let matches = App::new("BamBaseReader")
    .version("0.1.0")
    .author("Nacho Garcia <iggl@fhi.no>")
    .about("Base Extraction of BAM Files")
    .arg(Arg::with_name("file")
             .short("f")
             .long("file")
             .takes_value(true)
             .help("BAM file"))

    .arg(Arg::with_name("position")
            .short("p")
            .long("position")
            .takes_value(true)
            .help("Position ro read"))
    .get_matches();

//matches.is_present("dyn_mode")

    let myfile = matches.value_of("file").unwrap();
    let mut bam = IndexedReader::from_path(&myfile).unwrap();
//cutoff for calling the secondary sequence
let pos_tr = matches.value_of("position");
let mut pos= 1;

match pos_tr {
    None => pos = 1,
    Some(ps) => {
        match ps.parse::<u32>() {
            Ok(ps2) => pos = ps2,
            Err(_) => pos = 1,
        }
    }
}

    //bam.fetch(("Spike", 1001u32 , 2004u32)).unwrap(); 
    let mut p = bam.pileup();
    p.set_max_depth(12000000u32); //Fixing maximum depth to 12Mreads
    let mut in_count = 0;
    let mut in_len = 0;
    for p2 in p{

        let pileup = p2.unwrap();
        let mut total_base = Vec::new();
   
        in_count =0;
        in_len=0;
        for alignment in pileup.alignments() {

            if pileup.pos() > pos {
                break;
            }
            if !alignment.is_del() && !alignment.is_refskip() && pileup.pos()+1 ==pos {
                if alignment.record().seq().len() > 0{
                    total_base.push(alignment.record().seq()[alignment.qpos().unwrap()]);
                    let s = format!("{:?}", &alignment.record().qname());
                    let b_id=alignment.record().seq()[alignment.qpos().unwrap()]; 
                    let mut b_out="N";
                    if b_id==65 {b_out="A";};
                    if b_id==84 {b_out="T";};
                    if b_id==67 {b_out="C";};
                    if b_id==71 {b_out="G";};
                    println!("{}\t{}\t{}", s, pileup.pos()+1,b_out);
                }
            }
            if alignment.is_del() && !alignment.is_refskip()  && pileup.pos()+1 ==pos{
                let s = format!("{:?}", &alignment.record().qname());
                println!("{}\t{}\t{}", s, pileup.pos()+1,"D");    
            }
            
           
            match alignment.indel() {
               
                bam::pileup::Indel::Ins(len) => {

                    if pileup.pos()+1 ==pos {
                        let sq = format!("{:?}", &alignment.record().qname());
                        println!("{}\t{}\t{:?}", sq, pileup.pos()+1,"I"); 
                    };
     
                },
                bam::pileup::Indel::Del(len2) => (),
                bam::pileup::Indel::None => (),
            }
            
            

        }
    }
}