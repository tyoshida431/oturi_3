use wasm_bindgen::prelude::*;

// お金の枚数。
struct MoneyNumber{
    ju: i32,
    goju: i32,
    hyaku: i32,
    gohyaku: i32,
    sen: i32,
    gosen: i32,
    ichiman: i32
}

// 同人誌に対して払う時の同人誌の価格と、
// その同人誌を買うシミュレーション上の人数。
struct Pay{
    price: i32,
    ninzu: i32
}

// 同人誌一種。
struct Hon{
    id: i32,
    price: i32,
    hanpusu: i32,
    hanpu_count: i32,
    hanpu_number_combination: i32,
    amari: i32
}

// 複数冊の同人誌を組み合わせて計算する、
// 計算用のデーター。
struct HonForCalc{
    price: i32,
    hanpusu: i32,
    moto_list: Vec<i32>    
}

// 計算結果。
struct CalcResult{
    money_number: MoneyNumber,
    hon_list: Vec<Hon>,
    end_flag: bool
}

#[wasm_bindgen] 
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen(module="/oturi_3_view.js")]
extern "C"{
    pub fn getHonList() -> js_sys::Array;
    pub fn setResult(juCount: i32,gojuCount: i32,hyakuCount: i32,gohyakuCount: i32,senenCount: i32,gosenenCount: i32);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works(){
        assert_eq!(2+2,4);
    }
    // 10
    #[test]
    fn ju_ju_test(){
       assert_eq!(test_ju_money_number(10),25);
    }
    #[test]
    fn ju_goju_test(){
        assert_eq!(test_goju_money_number(10),1);
    }
    #[test]
    fn ju_hyaku_test(){
        assert_eq!(test_hyaku_money_number(10),-4);
    }
    #[test]
    fn ju_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(10),0);
    }
    // 20
    #[test]
    fn niju_ju_test(){
        assert_eq!(test_ju_money_number(20),15);
    }
    #[test]
    fn niju_goju_test(){
        assert_eq!(test_goju_money_number(20),1);
    }
    #[test]
    fn niju_hyaku_test(){
        assert_eq!(test_hyaku_money_number(20),-4);
    }
    #[test]
    fn niju_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(20),0);
    } 
    // 30
    #[test]
    fn t_30_ju_test(){
        assert_eq!(test_ju_money_number(30),5);
    }
    #[test]
    fn t_30_goju_test(){
        assert_eq!(test_goju_money_number(30),1);
    }
    #[test]
    fn t_30_hyaku_test(){
        assert_eq!(test_hyaku_money_number(30),-4);
    }
    #[test]
    fn t_30_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(30),0);
    }
    // 40
    #[test]
    fn t_40_ju_test(){
        assert_eq!(test_ju_money_number(40),-5);
    }
    #[test]
    fn t_40_goju_test(){
        assert_eq!(test_goju_money_number(40),1);
    }
    #[test]
    fn t_40_hyaku_test(){
        assert_eq!(test_hyaku_money_number(40),-4);
    }
    #[test]
    fn t_40_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(40),0);
    }
    // 50
    #[test]
    fn t_50_ju_test(){
        assert_eq!(test_ju_money_number(50),0);
    }
    #[test]
    fn t_50_goju_test(){
        assert_eq!(test_goju_money_number(50),0);
    }
    #[test]
    fn t_50_hyaku_test(){
        assert_eq!(test_hyaku_money_number(50),-5);
    }
    #[test]
    fn t_50_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(50),0);
    }
    // 60
    #[test]
    fn t_60_ju_test(){
       assert_eq!(test_ju_money_number(60),15);
    }
    #[test]
    fn t_60_goju_test(){
       assert_eq!(test_goju_money_number(60),-5);
    }
    #[test]
    fn t_60_hyaku_test(){
       assert_eq!(test_hyaku_money_number(60),-5);
    }
    #[test]
    fn t_60_gohyaku_test(){
       assert_eq!(test_gohyaku_money_number(60),0);
    }
    // 70
    #[test]
    fn t_70_ju_test(){
       assert_eq!(test_ju_money_number(70),5);
    }
    #[test]
    fn t_70_goju_test(){
       assert_eq!(test_goju_money_number(70),-5);
    }
    #[test]
    fn t_70_hyaku_test(){
       assert_eq!(test_hyaku_money_number(70),-5);
    }
    #[test]
    fn t_70_gohyaku_test(){
       assert_eq!(test_gohyaku_money_number(70),0);
    }
    // 80
    #[test]
    fn t_80_ju_test(){
       assert_eq!(test_ju_money_number(80),-5);
    }
    #[test]
    fn t_80_goju_test(){
       assert_eq!(test_goju_money_number(80),-5);
    }
    #[test]
    fn t_80_hyaku_test(){
       assert_eq!(test_hyaku_money_number(80),-5);
    }
    #[test]
    fn t_80_gohyaku_test(){
       assert_eq!(test_gohyaku_money_number(80),0);
    }
    // 90
    #[test]
    fn t_90_ju_test(){
       assert_eq!(test_ju_money_number(90),-15);
    }
    #[test]
    fn t_90_goju_test(){
       assert_eq!(test_goju_money_number(90),-5);
    }
    #[test]
    fn t_90_hyaku_test(){
       assert_eq!(test_hyaku_money_number(90),-5);
    }
    #[test]
    fn t_90_gohyaku_test(){
       assert_eq!(test_gohyaku_money_number(90),0);
    }
    // 100
    #[test]
    fn t_100_ju_test(){
        assert_eq!(test_ju_money_number(100),0);
    }
    #[test]
    fn t_100_goju_test(){
        assert_eq!(test_goju_money_number(100),0);
    }
    #[test]
    fn t_100_hyaku_test(){
        assert_eq!(test_hyaku_money_number(100),25);
    }
    #[test]
    fn t_100_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(100),1);
    }
    // 110
    #[test]
    fn t_110_ju_test(){
        assert_eq!(test_ju_money_number(110),30);
    }
    #[test]
    fn t_110_goju_test(){
        assert_eq!(test_goju_money_number(110),8);
    }
    #[test]
    fn t_110_hyaku_test(){
        assert_eq!(test_hyaku_money_number(110),12);
    }
    #[test]
    fn t_110_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(110),2);
    }
    // 120
    #[test]
    fn t_120_ju_test(){
        assert_eq!(test_ju_money_number(120),20);
    }
    #[test]
    fn t_120_goju_test(){
        assert_eq!(test_goju_money_number(120),8);
    }
    #[test]
    fn t_120_hyaku_test(){
        assert_eq!(test_hyaku_money_number(120),12);
    }
    #[test]
    fn t_120_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(120),2);
    }
    // 130
    #[test]
    fn t_130_ju_test(){
        assert_eq!(test_ju_money_number(130),10);
    }
    #[test]
    fn t_130_goju_test(){
        assert_eq!(test_goju_money_number(130),8);
    }
    #[test]
    fn t_130_hyaku_test(){
        assert_eq!(test_hyaku_money_number(130),12);
    }
    #[test]
    fn t_130_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(130),2);
    }
    // 140
    #[test]
    fn t_140_ju_test(){
        assert_eq!(test_ju_money_number(140),0);
    }
    #[test]
    fn t_140_goju_test(){
        assert_eq!(test_goju_money_number(140),8);
    }
    #[test]
    fn t_140_hyaku_test(){
        assert_eq!(test_hyaku_money_number(140),12);
    }
    #[test]
    fn t_140_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(140),2);
    }
    // 150
    #[test]
    fn t_150_ju_test(){
        assert_eq!(test_ju_money_number(150),0);
    }
    #[test]
    fn t_150_goju_test(){
        assert_eq!(test_goju_money_number(150),6);
    }
    #[test]
    fn t_150_hyaku_test(){
        assert_eq!(test_hyaku_money_number(150),12);
    }
    #[test]
    fn t_150_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(150),2);
    }
    // 160
    #[test]
    fn t_160_ju_test(){
        assert_eq!(test_ju_money_number(160),30);
    }
    #[test]
    fn t_160_goju_test(){
        assert_eq!(test_goju_money_number(160),-2);
    }
    #[test]
    fn t_160_hyaku_test(){
        assert_eq!(test_hyaku_money_number(160),12);
    }
    #[test]
    fn t_160_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(160),2);
    }
    // 170
    #[test]
    fn t_170_ju_test(){
        assert_eq!(test_ju_money_number(170),20);
    }
    #[test]
    fn t_170_goju_test(){
        assert_eq!(test_goju_money_number(170),-2);
    }
    #[test]
    fn t_170_hyaku_test(){
        assert_eq!(test_hyaku_money_number(170),12);
    }
    #[test]
    fn t_170_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(170),2);
    }
    // 180
    #[test]
    fn t_180_ju_test(){
        assert_eq!(test_ju_money_number(180),10);
    }
    #[test]
    fn t_180_goju_test(){
        assert_eq!(test_goju_money_number(180),-2);
    }
    #[test]
    fn t_180_hyaku_test(){
        assert_eq!(test_hyaku_money_number(180),12);
    }
    #[test]
    fn t_180_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(180),2);
    }
    // 190
    #[test]
    fn t_190_ju_test(){
        assert_eq!(test_ju_money_number(190),0);
    }
    #[test]
    fn t_190_goju_test(){
        assert_eq!(test_goju_money_number(190),-2);
    }
    #[test]
    fn t_190_hyaku_test(){
        assert_eq!(test_hyaku_money_number(190),12);
    }
    #[test]
    fn t_190_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(190),2);
    }
    // 200
    #[test]
    fn t_200_ju_test(){
        assert_eq!(test_ju_money_number(200),0);
    }
    #[test]
    fn t_200_goju_test(){
        assert_eq!(test_goju_money_number(200),0);
    }
    #[test]
    fn t_200_hyaku_test(){
        assert_eq!(test_hyaku_money_number(200),15);
    }
    #[test]
    fn t_200_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(200),1);
    }
    // 210
    #[test]
    fn t_210_ju_test(){
        assert_eq!(test_ju_money_number(210),30);
    }
    #[test]
    fn t_210_goju_test(){
        assert_eq!(test_goju_money_number(210),8);
    }
    #[test]
    fn t_210_hyaku_test(){
        assert_eq!(test_hyaku_money_number(210),2);
    }
    #[test]
    fn t_210_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(210),2);
    }
    // 220
    #[test]
    fn t_220_ju_test(){
        assert_eq!(test_ju_money_number(220),20);
    }
    #[test]
    fn t_220_goju_test(){
        assert_eq!(test_goju_money_number(220),8);
    }
    #[test]
    fn t_220_hyaku_test(){
        assert_eq!(test_hyaku_money_number(220),2);
    }
    #[test]
    fn t_220_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(220),2);
    }
    // 230
    #[test]
    fn t_230_ju_test(){
        assert_eq!(test_ju_money_number(230),10);
    }
    #[test]
    fn t_230_goju_test(){
        assert_eq!(test_goju_money_number(230),8);
    }
    #[test]
    fn t_230_hyaku_test(){
        assert_eq!(test_hyaku_money_number(230),2);
    }
    #[test]
    fn t_230_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(230),2);
    }
    // 240
    #[test]
    fn t_240_ju_test(){
        assert_eq!(test_ju_money_number(240),0);
    }
    #[test]
    fn t_240_goju_test(){
        assert_eq!(test_goju_money_number(240),8);
    }
    #[test]
    fn t_240_hyaku_test(){
        assert_eq!(test_hyaku_money_number(240),2);
    }
    #[test]
    fn t_240_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(240),2);
    }
    // 250
    #[test]
    fn t_250_ju_test(){
        assert_eq!(test_ju_money_number(250),0);
    }
    #[test]
    fn t_250_goju_test(){
        assert_eq!(test_goju_money_number(250),6);
    }
    #[test]
    fn t_250_hyaku_test(){
        assert_eq!(test_hyaku_money_number(250),2);
    }
    #[test]
    fn t_250_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(250),2);
    }
    // 260
    #[test]
    fn t_260_ju_test(){
        assert_eq!(test_ju_money_number(260),30);
    }
    #[test]
    fn t_260_goju_test(){
        assert_eq!(test_goju_money_number(260),-2);
    }
    #[test]
    fn t_260_hyaku_test(){
        assert_eq!(test_hyaku_money_number(260),2);
    }
    #[test]
    fn t_260_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(260),2);
    }
    // 270
    #[test]
    fn t_270_ju_test(){
        assert_eq!(test_ju_money_number(270),20);
    }
    #[test]
    fn t_270_goju_test(){
        assert_eq!(test_goju_money_number(270),-2);
    }
    #[test]
    fn t_270_hyaku_test(){
        assert_eq!(test_hyaku_money_number(270),2);
    }
    #[test]
    fn t_270_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(270),2);
    }
    // 280
    #[test]
    fn t_280_ju_test(){
        assert_eq!(test_ju_money_number(280),10);
    }
    #[test]
    fn t_280_goju_test(){
        assert_eq!(test_goju_money_number(280),-2);
    }
    #[test]
    fn t_280_hyaku_test(){
        assert_eq!(test_hyaku_money_number(280),2);
    }
    #[test]
    fn t_280_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(280),2);
    }
    // 290
    #[test]
    fn t_290_ju_test(){
        assert_eq!(test_ju_money_number(290),0);
    }
    #[test]
    fn t_290_goju_test(){
        assert_eq!(test_goju_money_number(290),-2);
    }
    #[test]
    fn t_290_hyaku_test(){
        assert_eq!(test_hyaku_money_number(290),2);
    }
    #[test]
    fn t_290_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(290),2);
    }
    // 300
    #[test]
    fn t_300_ju_test(){
        assert_eq!(test_ju_money_number(300),0);
    }
    #[test]
    fn t_300_goju_test(){
        assert_eq!(test_goju_money_number(300),0);
    }
    #[test]
    fn t_300_hyaku_test(){
        assert_eq!(test_hyaku_money_number(300),5);
    }
    #[test]
    fn t_300_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(300),1);
    }
    // 310
    #[test]
    fn t_310_ju_test(){
        assert_eq!(test_ju_money_number(310),30);
    }
    #[test]
    fn t_310_goju_test(){
        assert_eq!(test_goju_money_number(310),8);
    }
    #[test]
    fn t_310_hyaku_test(){
        assert_eq!(test_hyaku_money_number(310),-8);
    }
    #[test]
    fn t_310_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(310),2);
    }
    // 320
    #[test]
    fn t_320_ju_test(){
        assert_eq!(test_ju_money_number(320),20);
    }
    #[test]
    fn t_320_goju_test(){
        assert_eq!(test_goju_money_number(320),8);
    }
    #[test]
    fn t_320_hyaku_test(){
        assert_eq!(test_hyaku_money_number(320),-8);
    }
    #[test]
    fn t_320_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(320),2);
    }
    // 330
    #[test]
    fn t_330_ju_test(){
        assert_eq!(test_ju_money_number(330),10);
    }
    #[test]
    fn t_330_goju_test(){
        assert_eq!(test_goju_money_number(330),8);
    }
    #[test]
    fn t_330_hyaku_test(){
        assert_eq!(test_hyaku_money_number(330),-8);
    }
    #[test]
    fn t_330_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(330),2);
    }
    // 340
    #[test]
    fn t_340_ju_test(){
        assert_eq!(test_ju_money_number(340),0);
    }
    #[test]
    fn t_340_goju_test(){
        assert_eq!(test_goju_money_number(340),8);
    }
    #[test]
    fn t_340_hyaku_test(){
        assert_eq!(test_hyaku_money_number(340),-8);
    }
    #[test]
    fn t_340_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(340),2);
    }
    // 350
    #[test]
    fn t_350_ju_test(){
        assert_eq!(test_ju_money_number(350),0);
    }
    #[test]
    fn t_350_goju_test(){
        assert_eq!(test_goju_money_number(350),6);
    }
    #[test]
    fn t_350_hyaku_test(){
        assert_eq!(test_hyaku_money_number(350),-8);
    }
    #[test]
    fn t_350_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(350),2);
    }
    // 360
    #[test]
    fn t_360_ju_test(){
        assert_eq!(test_ju_money_number(360),30);
    }
    #[test]
    fn t_360_goju_test(){
        assert_eq!(test_goju_money_number(360),-2);
    }
    #[test]
    fn t_360_hyaku_test(){
        assert_eq!(test_hyaku_money_number(360),-8);
    }
    #[test]
    fn t_360_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(360),2);
    }
    // 370
    #[test]
    fn t_370_ju_test(){
        assert_eq!(test_ju_money_number(370),20);
    }
    #[test]
    fn t_370_goju_test(){
        assert_eq!(test_goju_money_number(370),-2);
    }
    #[test]
    fn t_370_hyaku_test(){
        assert_eq!(test_hyaku_money_number(370),-8);
    }
    #[test]
    fn t_370_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(370),2);
    }
    // 380
    #[test]
    fn t_380_ju_test(){
        assert_eq!(test_ju_money_number(380),10);
    }
    #[test]
    fn t_380_goju_test(){
        assert_eq!(test_goju_money_number(380),-2);
    }
    #[test]
    fn t_380_hyaku_test(){
        assert_eq!(test_hyaku_money_number(380),-8);
    }
    #[test]
    fn t_380_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(380),2);
    }
    // 390
    #[test]
    fn t_390_ju_test(){
        assert_eq!(test_ju_money_number(390),0);
    }
    #[test]
    fn t_390_goju_test(){
        assert_eq!(test_goju_money_number(390),-2);
    }
    #[test]
    fn t_390_hyaku_test(){
        assert_eq!(test_hyaku_money_number(390),-8);
    }
    #[test]
    fn t_390_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(390),2);
    }
    // 400
    #[test]
    fn t_400_ju_test(){
        assert_eq!(test_ju_money_number(400),0);
    }
    #[test]
    fn t_400_goju_test(){
        assert_eq!(test_goju_money_number(400),0);
    }
    #[test]
    fn t_400_hyaku_test(){
        assert_eq!(test_hyaku_money_number(400),-5);
    }
    #[test]
    fn t_400_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(400),1);
    }
    // 410
    #[test]
    fn t_410_ju_test(){
        assert_eq!(test_ju_money_number(410),25);
    }
    #[test]
    fn t_410_goju_test(){
        assert_eq!(test_goju_money_number(410),7);
    }
    #[test]
    fn t_410_hyaku_test(){
        assert_eq!(test_hyaku_money_number(410),-12);
    }
    #[test]
    fn t_410_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(410),1);
    }
    // 420
    #[test]
    fn t_420_ju_test(){
        assert_eq!(test_ju_money_number(420),15);
    }
    #[test]
    fn t_420_goju_test(){
        assert_eq!(test_goju_money_number(420),7);
    }
    #[test]
    fn t_420_hyaku_test(){
        assert_eq!(test_hyaku_money_number(420),-12);
    }
    #[test]
    fn t_420_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(420),1);
    }
    // 430
    #[test]
    fn t_430_ju_test(){
        assert_eq!(test_ju_money_number(430),5);
    }
    #[test]
    fn t_430_goju_test(){
        assert_eq!(test_goju_money_number(430),7);
    }
    #[test]
    fn t_430_hyaku_test(){
        assert_eq!(test_hyaku_money_number(430),-12);
    }
    #[test]
    fn t_430_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(430),1);
    }
    // 440
    #[test]
    fn t_440_ju_test(){
        assert_eq!(test_ju_money_number(440),-5);
    }
    #[test]
    fn t_440_goju_test(){
        assert_eq!(test_goju_money_number(440),7);
    }
    #[test]
    fn t_440_hyaku_test(){
        assert_eq!(test_hyaku_money_number(440),-12);
    }
    #[test]
    fn t_440_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(440),1);
    }
    // 450
    #[test]
    fn t_450_ju_test(){
        assert_eq!(test_ju_money_number(450),0);
    }
    #[test]
    fn t_450_goju_test(){
        assert_eq!(test_goju_money_number(450),4);
    }
    #[test]
    fn t_450_hyaku_test(){
        assert_eq!(test_hyaku_money_number(450),-12);
    }
    #[test]
    fn t_450_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(450),1);
    }
    // 460
    #[test]
    fn t_460_ju_test(){
        assert_eq!(test_ju_money_number(460),25);
    }
    #[test]
    fn t_460_goju_test(){
        assert_eq!(test_goju_money_number(460),-3);
    }
    #[test]
    fn t_460_hyaku_test(){
        assert_eq!(test_hyaku_money_number(460),-12);
    }
    #[test]
    fn t_460_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(460),1);
    }
    // 470
    #[test]
    fn t_470_ju_test(){
        assert_eq!(test_ju_money_number(470),15);
    }
    #[test]
    fn t_470_goju_test(){
        assert_eq!(test_goju_money_number(470),-3);
    }
    #[test]
    fn t_470_hyaku_test(){
        assert_eq!(test_hyaku_money_number(470),-12);
    }
    #[test]
    fn t_470_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(470),1);
    }
    // 480
    #[test]
    fn t_480_ju_test(){
        assert_eq!(test_ju_money_number(480),5);
    }
    #[test]
    fn t_480_goju_test(){
        assert_eq!(test_goju_money_number(480),-3);
    }
    #[test]
    fn t_480_hyaku_test(){
        assert_eq!(test_hyaku_money_number(480),-12);
    }
    #[test]
    fn t_480_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(480),1);
    }
    // 490
    #[test]
    fn t_490_ju_test(){
        assert_eq!(test_ju_money_number(490),-5);
    }
    #[test]
    fn t_490_goju_test(){
        assert_eq!(test_goju_money_number(490),-3);
    }
    #[test]
    fn t_490_hyaku_test(){
        assert_eq!(test_hyaku_money_number(490),-12);
    }
    #[test]
    fn t_490_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(490),1);
    }
    // 500
    #[test]
    fn t_500_ju_test(){
        assert_eq!(test_ju_money_number(500),0);
    }
    #[test]
    fn t_500_goju_test(){
        assert_eq!(test_goju_money_number(500),0);
    }
    #[test]
    fn t_500_hyaku_test(){
        assert_eq!(test_hyaku_money_number(500),0);
    }
    #[test]
    fn t_500_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(500),0);
    }
    // 510
    #[test]
    fn t_510_ju_test(){
        assert_eq!(test_ju_money_number(510),15);
    }
    #[test]
    fn t_510_goju_test(){
        assert_eq!(test_goju_money_number(510),5);
    }
    #[test]
    fn t_510_hyaku_test(){
        assert_eq!(test_hyaku_money_number(510),20);
    }
    #[test]
    fn t_510_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(510),-5);
    }
    // 520
    #[test]
    fn t_520_ju_test(){
        assert_eq!(test_ju_money_number(520),5);
    }
    #[test]
    fn t_520_goju_test(){
        assert_eq!(test_goju_money_number(520),5);
    }
    #[test]
    fn t_520_hyaku_test(){
        assert_eq!(test_hyaku_money_number(520),20);
    }
    #[test]
    fn t_520_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(520),-5);
    }
    // 530
    #[test]
    fn t_530_ju_test(){
        assert_eq!(test_ju_money_number(530),-5);
    }
    #[test]
    fn t_530_goju_test(){
        assert_eq!(test_goju_money_number(530),5);
    }
    #[test]
    fn t_530_hyaku_test(){
        assert_eq!(test_hyaku_money_number(530),20);
    }
    #[test]
    fn t_530_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(530),-5);
    }
    // 540
    #[test]
    fn t_540_ju_test(){
        assert_eq!(test_ju_money_number(540),-15);
    }
    #[test]
    fn t_540_goju_test(){
        assert_eq!(test_goju_money_number(540),5);
    }
    #[test]
    fn t_540_hyaku_test(){
        assert_eq!(test_hyaku_money_number(540),20);
    }
    #[test]
    fn t_540_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(540),-5);
    }
    // 550
    #[test]
    fn t_550_ju_test(){
        assert_eq!(test_ju_money_number(550),0);
    }
    #[test]
    fn t_550_goju_test(){
        assert_eq!(test_goju_money_number(550),0);
    }
    #[test]
    fn t_550_hyaku_test(){
        assert_eq!(test_hyaku_money_number(550),20);
    }
    #[test]
    fn t_550_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(550),-5);
    }
    // 560
    #[test]
    fn t_560_ju_test(){
        assert_eq!(test_ju_money_number(560),15);
    }
    #[test]
    fn t_560_goju_test(){
        assert_eq!(test_goju_money_number(560),-5);
    }
    #[test]
    fn t_560_hyaku_test(){
        assert_eq!(test_hyaku_money_number(560),20);
    }
    #[test]
    fn t_560_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(560),-5);
    }
    // 570
    #[test]
    fn t_570_ju_test(){
        assert_eq!(test_ju_money_number(570),5);
    }
    #[test]
    fn t_570_goju_test(){
        assert_eq!(test_goju_money_number(570),-5);
    }
    #[test]
    fn t_570_hyaku_test(){
        assert_eq!(test_hyaku_money_number(570),20);
    }
    #[test]
    fn t_570_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(570),-5);
    }
    // 580
    #[test]
    fn t_580_ju_test(){
        assert_eq!(test_ju_money_number(580),-5);
    }
    #[test]
    fn t_580_goju_test(){
        assert_eq!(test_goju_money_number(580),-5);
    }
    #[test]
    fn t_580_hyaku_test(){
        assert_eq!(test_hyaku_money_number(580),20);
    }
    #[test]
    fn t_580_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(580),-5);
    }
    // 590
    #[test]
    fn t_590_ju_test(){
        assert_eq!(test_ju_money_number(590),-15);
    }
    #[test]
    fn t_590_goju_test(){
        assert_eq!(test_goju_money_number(590),-5);
    }
    #[test]
    fn t_590_hyaku_test(){
        assert_eq!(test_hyaku_money_number(590),20);
    }
    #[test]
    fn t_590_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(590),-5);
    }
    // 600
    #[test]
    fn t_600_ju_test(){
        assert_eq!(test_ju_money_number(600),0);
    }
    #[test]
    fn t_600_goju_test(){
        assert_eq!(test_goju_money_number(600),0);
    }
    #[test]
    fn t_600_hyaku_test(){
        assert_eq!(test_hyaku_money_number(600),15);
    }
    #[test]
    fn t_600_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(600),-5);
    }
    // 610
    #[test]
    fn t_610_ju_test(){
        assert_eq!(test_ju_money_number(610),15);
    }
    #[test]
    fn t_610_goju_test(){
        assert_eq!(test_goju_money_number(610),5);
    }
    #[test]
    fn t_610_hyaku_test(){
        assert_eq!(test_hyaku_money_number(610),10);
    }
    #[test]
    fn t_610_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(610),-5);
    }
    // 620
    #[test]
    fn t_620_ju_test(){
        assert_eq!(test_ju_money_number(620),5);
    }
    #[test]
    fn t_620_goju_test(){
        assert_eq!(test_goju_money_number(620),5);
    }
    #[test]
    fn t_620_hyaku_test(){
        assert_eq!(test_hyaku_money_number(620),10);
    }
    #[test]
    fn t_620_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(620),-5);
    }
    // 630
    #[test]
    fn t_630_ju_test(){
        assert_eq!(test_ju_money_number(630),-5);
    }
    #[test]
    fn t_630_goju_test(){
        assert_eq!(test_goju_money_number(630),5);
    }
    #[test]
    fn t_630_hyaku_test(){
        assert_eq!(test_hyaku_money_number(630),10);
    }
    #[test]
    fn t_630_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(630),-5);
    }
    // 640
    #[test]
    fn t_640_ju_test(){
        assert_eq!(test_ju_money_number(640),-15);
    }
    #[test]
    fn t_640_goju_test(){
        assert_eq!(test_goju_money_number(640),5);
    }
    #[test]
    fn t_640_hyaku_test(){
        assert_eq!(test_hyaku_money_number(640),10);
    }
    #[test]
    fn t_640_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(640),-5);
    }
    // 650
    #[test]
    fn t_650_ju_test(){
        assert_eq!(test_ju_money_number(650),0);
    }
    #[test]
    fn t_650_goju_test(){
        assert_eq!(test_goju_money_number(650),0);
    }
    #[test]
    fn t_650_hyaku_test(){
        assert_eq!(test_hyaku_money_number(650),10);
    }
    #[test]
    fn t_650_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(650),-5);
    }
    // 660
    #[test]
    fn t_660_ju_test(){
        assert_eq!(test_ju_money_number(660),15);
    }
    #[test]
    fn t_660_goju_test(){
        assert_eq!(test_goju_money_number(660),-5);
    }
    #[test]
    fn t_660_hyaku_test(){
        assert_eq!(test_hyaku_money_number(660),10);
    }
    #[test]
    fn t_660_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(660),-5);
    }
    // 670
    #[test]
    fn t_670_ju_test(){
        assert_eq!(test_ju_money_number(670),5);
    }
    #[test]
    fn t_670_goju_test(){
        assert_eq!(test_goju_money_number(670),-5);
    }
    #[test]
    fn t_670_hyaku_test(){
        assert_eq!(test_hyaku_money_number(670),10);
    }
    #[test]
    fn t_670_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(670),-5);
    }
    // 680
    #[test]
    fn t_680_ju_test(){
        assert_eq!(test_ju_money_number(680),-5);
    }
    #[test]
    fn t_680_goju_test(){
        assert_eq!(test_goju_money_number(680),-5);
    }
    #[test]
    fn t_680_hyaku_test(){
        assert_eq!(test_hyaku_money_number(680),10);
    }
    #[test]
    fn t_680_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(680),-5);
    }
    // 690
    #[test]
    fn t_690_ju_test(){
        assert_eq!(test_ju_money_number(690),-15);
    }
    #[test]
    fn t_690_goju_test(){
        assert_eq!(test_goju_money_number(690),-5);
    }
    #[test]
    fn t_690_hyaku_test(){
        assert_eq!(test_hyaku_money_number(690),10);
    }
    #[test]
    fn t_690_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(690),-5);
    }
    // 700
    #[test]
    fn t_700_ju_test(){
        assert_eq!(test_ju_money_number(700),0);
    }
    #[test]
    fn t_700_goju_test(){
        assert_eq!(test_goju_money_number(700),0);
    }
    #[test]
    fn t_700_hyaku_test(){
        assert_eq!(test_hyaku_money_number(700),5);
    }
    #[test]
    fn t_700_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(700),-5);
    }
    // 710
    #[test]
    fn t_710_ju_test(){
        assert_eq!(test_ju_money_number(710),15);
    }
    #[test]
    fn t_710_goju_test(){
        assert_eq!(test_goju_money_number(710),5);
    }
    #[test]
    fn t_710_hyaku_test(){
        assert_eq!(test_hyaku_money_number(710),0);
    }
    #[test]
    fn t_710_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(710),-5);
    }
    // 720
    #[test]
    fn t_720_ju_test(){
        assert_eq!(test_ju_money_number(720),5);
    }
    #[test]
    fn t_720_goju_test(){
        assert_eq!(test_goju_money_number(720),5);
    }
    #[test]
    fn t_720_hyaku_test(){
        assert_eq!(test_hyaku_money_number(720),0);
    }
    #[test]
    fn t_720_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(720),-5);
    }
    // 730
    #[test]
    fn t_730_ju_test(){
        assert_eq!(test_ju_money_number(730),-5);
    }
    #[test]
    fn t_730_goju_test(){
        assert_eq!(test_goju_money_number(730),5);
    }
    #[test]
    fn t_730_hyaku_test(){
        assert_eq!(test_hyaku_money_number(730),0);
    }
    #[test]
    fn t_730_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(730),-5);
    }
    // 740
    #[test]
    fn t_740_ju_test(){
        assert_eq!(test_ju_money_number(740),-15);
    }
    #[test]
    fn t_740_goju_test(){
        assert_eq!(test_goju_money_number(740),5);
    }
    #[test]
    fn t_740_hyaku_test(){
        assert_eq!(test_hyaku_money_number(740),0);
    }
    #[test]
    fn t_740_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(740),-5);
    }
    // 750
    #[test]
    fn t_750_ju_test(){
        assert_eq!(test_ju_money_number(750),0);
    }
    #[test]
    fn t_750_goju_test(){
        assert_eq!(test_goju_money_number(750),0);
    }
    #[test]
    fn t_750_hyaku_test(){
        assert_eq!(test_hyaku_money_number(750),0);
    }
    #[test]
    fn t_750_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(750),-5);
    }
    // 760
    #[test]
    fn t_760_ju_test(){
        assert_eq!(test_ju_money_number(760),15);
    }
    #[test]
    fn t_760_goju_test(){
        assert_eq!(test_goju_money_number(760),-5);
    }
    #[test]
    fn t_760_hyaku_test(){
        assert_eq!(test_hyaku_money_number(760),0);
    }
    #[test]
    fn t_760_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(760),-5);
    }
    // 770
    #[test]
    fn t_770_ju_test(){
        assert_eq!(test_ju_money_number(770),5);
    }
    #[test]
    fn t_770_goju_test(){
        assert_eq!(test_goju_money_number(770),-5);
    }
    #[test]
    fn t_770_hyaku_test(){
        assert_eq!(test_hyaku_money_number(770),0);
    }
    #[test]
    fn t_770_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(770),-5);
    }
    // 780
    #[test]
    fn t_780_ju_test(){
        assert_eq!(test_ju_money_number(780),-5);
    }
    #[test]
    fn t_780_goju_test(){
        assert_eq!(test_goju_money_number(780),-5);
    }
    #[test]
    fn t_780_hyaku_test(){
        assert_eq!(test_hyaku_money_number(780),0);
    }
    #[test]
    fn t_780_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(780),-5);
    }
    // 790
    #[test]
    fn t_790_ju_test(){
        assert_eq!(test_ju_money_number(790),-15);
    }
    #[test]
    fn t_790_goju_test(){
        assert_eq!(test_goju_money_number(790),-5);
    }
    #[test]
    fn t_790_hyaku_test(){
        assert_eq!(test_hyaku_money_number(790),0);
    }
    #[test]
    fn t_790_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(790),-5);
    }
    // 800
    #[test]
    fn t_800_ju_test(){
        assert_eq!(test_ju_money_number(800),0);
    }
    #[test]
    fn t_800_goju_test(){
        assert_eq!(test_goju_money_number(800),0);
    }
    #[test]
    fn t_800_hyaku_test(){
        assert_eq!(test_hyaku_money_number(800),-5);
    }
    #[test]
    fn t_800_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(800),-5);
    }
    // 810
    #[test]
    fn t_810_ju_test(){
        assert_eq!(test_ju_money_number(810),15);
    }
    #[test]
    fn t_810_goju_test(){
        assert_eq!(test_goju_money_number(810),5);
    }
    #[test]
    fn t_810_hyaku_test(){
        assert_eq!(test_hyaku_money_number(810),-10);
    }
    #[test]
    fn t_810_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(810),-5);
    }
    // 820
    #[test]
    fn t_820_ju_test(){
        assert_eq!(test_ju_money_number(820),5);
    }
    #[test]
    fn t_820_goju_test(){
        assert_eq!(test_goju_money_number(820),5);
    }
    #[test]
    fn t_820_hyaku_test(){
        assert_eq!(test_hyaku_money_number(820),-10);
    }
    #[test]
    fn t_820_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(820),-5);
    }
    // 830
    #[test]
    fn t_830_ju_test(){
        assert_eq!(test_ju_money_number(830),-5);
    }
    #[test]
    fn t_830_goju_test(){
        assert_eq!(test_goju_money_number(830),5);
    }
    #[test]
    fn t_830_hyaku_test(){
        assert_eq!(test_hyaku_money_number(830),-10);
    }
    #[test]
    fn t_830_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(830),-5);
    }
    // 840
    #[test]
    fn t_840_ju_test(){
        assert_eq!(test_ju_money_number(840),-15);
    }
    #[test]
    fn t_840_goju_test(){
        assert_eq!(test_goju_money_number(840),5);
    }
    #[test]
    fn t_840_hyaku_test(){
        assert_eq!(test_hyaku_money_number(840),-10);
    }
    #[test]
    fn t_840_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(840),-5);
    }
    // 850
    #[test]
    fn t_850_ju_test(){
        assert_eq!(test_ju_money_number(850),0);
    }
    #[test]
    fn t_850_goju_test(){
        assert_eq!(test_goju_money_number(850),0);
    }
    #[test]
    fn t_850_hyaku_test(){
        assert_eq!(test_hyaku_money_number(850),-10);
    }
    #[test]
    fn t_850_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(850),-5);
    }
    // 860
    #[test]
    fn t_860_ju_test(){
        assert_eq!(test_ju_money_number(860),15);
    }
    #[test]
    fn t_860_goju_test(){
        assert_eq!(test_goju_money_number(860),-5);
    }
    #[test]
    fn t_860_hyaku_test(){
        assert_eq!(test_hyaku_money_number(860),-10);
    }
    #[test]
    fn t_860_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(860),-5);
    }
    // 870
    #[test]
    fn t_870_ju_test(){
        assert_eq!(test_ju_money_number(870),5);
    }
    #[test]
    fn t_870_goju_test(){
        assert_eq!(test_goju_money_number(870),-5);
    }
    #[test]
    fn t_870_hyaku_test(){
        assert_eq!(test_hyaku_money_number(870),-10);
    }
    #[test]
    fn t_870_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(870),-5);
    }
    // 880
    #[test]
    fn t_880_ju_test(){
        assert_eq!(test_ju_money_number(880),-5);
    }
    #[test]
    fn t_880_goju_test(){
        assert_eq!(test_goju_money_number(880),-5);
    }
    #[test]
    fn t_880_hyaku_test(){
        assert_eq!(test_hyaku_money_number(880),-10);
    }
    #[test]
    fn t_880_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(880),-5);
    }
    // 890
    #[test]
    fn t_890_ju_test(){
        assert_eq!(test_ju_money_number(890),-15);
    }
    #[test]
    fn t_890_goju_test(){
        assert_eq!(test_goju_money_number(890),-5);
    }
    #[test]
    fn t_890_hyaku_test(){
        assert_eq!(test_hyaku_money_number(890),-10);
    }
    #[test]
    fn t_890_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(890),-5);
    }
    // 900
    #[test]
    fn t_900_ju_test(){
        assert_eq!(test_ju_money_number(900),0);
    }
    #[test]
    fn t_900_goju_test(){
        assert_eq!(test_goju_money_number(900),0);
    }
    #[test]
    fn t_900_hyaku_test(){
        assert_eq!(test_hyaku_money_number(900),-15);
    }
    #[test]
    fn t_900_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(900),-5);
    }
    // 910
    #[test]
    fn t_910_ju_test(){
        assert_eq!(test_ju_money_number(910),15);
    }
    #[test]
    fn t_910_goju_test(){
        assert_eq!(test_goju_money_number(910),5);
    }
    #[test]
    fn t_910_hyaku_test(){
        assert_eq!(test_hyaku_money_number(910),-20);
    }
    #[test]
    fn t_910_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(910),-5);
    }
    // 920
    #[test]
    fn t_920_ju_test(){
        assert_eq!(test_ju_money_number(920),5);
    }
    #[test]
    fn t_920_goju_test(){
        assert_eq!(test_goju_money_number(920),5);
    }
    #[test]
    fn t_920_hyaku_test(){
        assert_eq!(test_hyaku_money_number(920),-20);
    }
    #[test]
    fn t_920_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(920),-5);
    }
    // 930
    #[test]
    fn t_930_ju_test(){
        assert_eq!(test_ju_money_number(930),-5);
    }
    #[test]
    fn t_930_goju_test(){
        assert_eq!(test_goju_money_number(930),5);
    }
    #[test]
    fn t_930_hyaku_test(){
        assert_eq!(test_hyaku_money_number(930),-20);
    }
    #[test]
    fn t_930_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(930),-5);
    }
    // 940
    #[test]
    fn t_940_ju_test(){
        assert_eq!(test_ju_money_number(940),-15);
    }
    #[test]
    fn t_940_goju_test(){
        assert_eq!(test_goju_money_number(940),5);
    }
    #[test]
    fn t_940_hyaku_test(){
        assert_eq!(test_hyaku_money_number(940),-20);
    }
    #[test]
    fn t_940_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(940),-5);
    }
    // 950
    #[test]
    fn t_950_ju_test(){
        assert_eq!(test_ju_money_number(950),0);
    }
    #[test]
    fn t_950_goju_test(){
        assert_eq!(test_goju_money_number(950),0);
    }
    #[test]
    fn t_950_hyaku_test(){
        assert_eq!(test_hyaku_money_number(950),-20);
    }
    #[test]
    fn t_950_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(950),-5);
    }
    // 960
    #[test]
    fn t_960_ju_test(){
        assert_eq!(test_ju_money_number(960),15);
    }
    #[test]
    fn t_960_goju_test(){
        assert_eq!(test_goju_money_number(960),-5);
    }
    #[test]
    fn t_960_hyaku_test(){
        assert_eq!(test_hyaku_money_number(960),-20);
    }
    #[test]
    fn t_960_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(960),-5);
    }
    // 970
    #[test]
    fn t_970_ju_test(){
        assert_eq!(test_ju_money_number(970),5);
    }
    #[test]
    fn t_970_goju_test(){
        assert_eq!(test_goju_money_number(970),-5);
    }
    #[test]
    fn t_970_hyaku_test(){
        assert_eq!(test_hyaku_money_number(970),-20);
    }
    #[test]
    fn t_970_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(970),-5);
    }
    // 980
    #[test]
    fn t_980_ju_test(){
        assert_eq!(test_ju_money_number(980),-5);
    }
    #[test]
    fn t_980_goju_test(){
        assert_eq!(test_goju_money_number(980),-5);
    }
    #[test]
    fn t_980_hyaku_test(){
        assert_eq!(test_hyaku_money_number(980),-20);
    }
    #[test]
    fn t_980_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(980),-5);
    }
    // 990
    #[test]
    fn t_990_ju_test(){
        assert_eq!(test_ju_money_number(990),-15);
    }
    #[test]
    fn t_990_goju_test(){
        assert_eq!(test_goju_money_number(990),-5);
    }
    #[test]
    fn t_990_hyaku_test(){
        assert_eq!(test_hyaku_money_number(990),-20);
    }
    #[test]
    fn t_990_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(990),-5);
    }
    // 1000
    #[test]
    fn t_1000_ju_test(){
        assert_eq!(test_ju_money_number(1000),0);
    }
    #[test]
    fn t_1000_goju_test(){
        assert_eq!(test_goju_money_number(1000),0);
    }
    #[test]
    fn t_1000_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1000),0);
    }
    #[test]
    fn t_1000_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1000),0);
    }
    // 1010
    #[test]
    fn t_1010_ju_test(){
        assert_eq!(test_ju_money_number(1010),25);
    }
    #[test]
    fn t_1010_goju_test(){
        assert_eq!(test_goju_money_number(1010),1);
    }
    #[test]
    fn t_1010_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1010),-4);
    }
    #[test]
    fn t_1010_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1010),0);
    }
    // 1020
    #[test]
    fn t_1020_ju_test(){
        assert_eq!(test_ju_money_number(1020),15);
    }
    #[test]
    fn t_1020_goju_test(){
        assert_eq!(test_goju_money_number(1020),1);
    }
    #[test]
    fn t_1020_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1020),-4);
    }
    #[test]
    fn t_1020_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1020),0);
    }
    // 1030
    #[test]
    fn t_1030_ju_test(){
        assert_eq!(test_ju_money_number(1030),5);
    }
    #[test]
    fn t_1030_goju_test(){
        assert_eq!(test_goju_money_number(1030),1);
    }
    #[test]
    fn t_1030_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1030),-4);
    }
    #[test]
    fn t_1030_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1030),0);
    }
    // 1040
    #[test]
    fn t_1040_ju_test(){
        assert_eq!(test_ju_money_number(1040),-5);
    }
    #[test]
    fn t_1040_goju_test(){
        assert_eq!(test_goju_money_number(1040),1);
    }
    #[test]
    fn t_1040_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1040),-4);
    }
    #[test]
    fn t_1040_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1040),0);
    }
    // 1050
    #[test]
    fn t_1050_ju_test(){
        assert_eq!(test_ju_money_number(1050),0);
    }
    #[test]
    fn t_1050_goju_test(){
        assert_eq!(test_goju_money_number(1050),0);
    }
    #[test]
    fn t_1050_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1050),-5);
    }
    #[test]
    fn t_1050_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1050),0);
    }
    // 1060
    #[test]
    fn t_1060_ju_test(){
        assert_eq!(test_ju_money_number(1060),15);
    }
    #[test]
    fn t_1060_goju_test(){
        assert_eq!(test_goju_money_number(1060),-5);
    }
    #[test]
    fn t_1060_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1060),-5);
    }
    #[test]
    fn t_1060_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1060),0);
    }
    // 1070
    #[test]
    fn t_1070_ju_test(){
        assert_eq!(test_ju_money_number(1070),5);
    }
    #[test]
    fn t_1070_goju_test(){
        assert_eq!(test_goju_money_number(1070),-5);
    }
    #[test]
    fn t_1070_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1070),-5);
    }
    #[test]
    fn t_1070_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1070),0);
    }
    // 1080
    #[test]
    fn t_1080_ju_test(){
        assert_eq!(test_ju_money_number(1080),-5);
    }
    #[test]
    fn t_1080_goju_test(){
        assert_eq!(test_goju_money_number(1080),-5);
    }
    #[test]
    fn t_1080_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1080),-5);
    }
    #[test]
    fn t_1080_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1080),0);
    }
    // 1090
    #[test]
    fn t_1090_ju_test(){
        assert_eq!(test_ju_money_number(1090),-15);
    }
    #[test]
    fn t_1090_goju_test(){
        assert_eq!(test_goju_money_number(1090),-5);
    }
    #[test]
    fn t_1090_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1090),-5);
    }
    #[test]
    fn t_1090_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1090),0);
    }
    // 1110
    #[test]
    fn t_1110_ju_test(){
        assert_eq!(test_ju_money_number(1110),30);
    }
    #[test]
    fn t_1110_goju_test(){
        assert_eq!(test_goju_money_number(1110),8);
    }
    #[test]
    fn t_1110_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1110),12);
    }
    #[test]
    fn t_1110_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1110),2);
    }
    // 1120
    #[test]
    fn t_1120_ju_test(){
        assert_eq!(test_ju_money_number(1120),20);
    }
    #[test]
    fn t_1120_goju_test(){
        assert_eq!(test_goju_money_number(1120),8);
    }
    #[test]
    fn t_1120_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1120),12);
    }
    #[test]
    fn t_1120_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1120),2);
    }
    // 1130
    #[test]
    fn t_1130_ju_test(){
        assert_eq!(test_ju_money_number(1130),10);
    }
    #[test]
    fn t_1130_goju_test(){
        assert_eq!(test_goju_money_number(1130),8);
    }
    #[test]
    fn t_1130_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1130),12);
    }
    #[test]
    fn t_1130_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1130),2);
    }
    // 1140
    #[test]
    fn t_1140_ju_test(){
        assert_eq!(test_ju_money_number(1140),0);
    }
    #[test]
    fn t_1140_goju_test(){
        assert_eq!(test_goju_money_number(1140),8);
    }
    #[test]
    fn t_1140_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1140),12);
    }
    #[test]
    fn t_1140_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1140),2);
    }
    // 1150
    #[test]
    fn t_1150_ju_test(){
        assert_eq!(test_ju_money_number(1150),0);
    }
    #[test]
    fn t_1150_goju_test(){
        assert_eq!(test_goju_money_number(1150),6);
    }
    #[test]
    fn t_1150_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1150),12);
    }
    #[test]
    fn t_1150_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1150),2);
    }
    // 1160
    #[test]
    fn t_1160_ju_test(){
        assert_eq!(test_ju_money_number(1160),30);
    }
    #[test]
    fn t_1160_goju_test(){
        assert_eq!(test_goju_money_number(1160),-2);
    }
    #[test]
    fn t_1160_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1160),12);
    }
    #[test]
    fn t_1160_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1160),2);
    }
    // 1170
    #[test]
    fn t_1170_ju_test(){
        assert_eq!(test_ju_money_number(1170),20);
    }
    #[test]
    fn t_1170_goju_test(){
        assert_eq!(test_goju_money_number(1170),-2);
    }
    #[test]
    fn t_1170_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1170),12);
    }
    #[test]
    fn t_1170_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1170),2);
    }
    // 1180
    #[test]
    fn t_1180_ju_test(){
        assert_eq!(test_ju_money_number(1180),10);
    }
    #[test]
    fn t_1180_goju_test(){
        assert_eq!(test_goju_money_number(1180),-2);
    }
    #[test]
    fn t_1180_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1180),12);
    }
    #[test]
    fn t_1180_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1180),2);
    }
    // 1190
    #[test]
    fn t_1190_ju_test(){
        assert_eq!(test_ju_money_number(1190),0);
    }
    #[test]
    fn t_1190_goju_test(){
        assert_eq!(test_goju_money_number(1190),-2);
    }
    #[test]
    fn t_1190_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1190),12);
    }
    #[test]
    fn t_1190_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1190),2);
    }
    // 1200
    #[test]
    fn t_1200_ju_test(){
        assert_eq!(test_ju_money_number(1200),0);
    }
    #[test]
    fn t_1200_goju_test(){
        assert_eq!(test_goju_money_number(1200),0);
    }
    #[test]
    fn t_1200_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1200),15);
    }
    #[test]
    fn t_1200_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1200),1);
    }
    // 1210
    #[test]
    fn t_1210_ju_test(){
        assert_eq!(test_ju_money_number(1210),30);
    }
    #[test]
    fn t_1210_goju_test(){
        assert_eq!(test_goju_money_number(1210),8);
    }
    #[test]
    fn t_1210_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1210),2);
    }
    #[test]
    fn t_1210_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1210),2);
    }
    // 1220
    #[test]
    fn t_1220_ju_test(){
        assert_eq!(test_ju_money_number(1220),20);
    }
    #[test]
    fn t_1220_goju_test(){
        assert_eq!(test_goju_money_number(1220),8);
    }
    #[test]
    fn t_1220_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1220),2);
    }
    #[test]
    fn t_1220_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1220),2);
    }
    // 1230
    #[test]
    fn t_1230_ju_test(){
        assert_eq!(test_ju_money_number(1230),10);
    }
    #[test]
    fn t_1230_goju_test(){
        assert_eq!(test_goju_money_number(1230),8);
    }
    #[test]
    fn t_1230_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1230),2);
    }
    #[test]
    fn t_1230_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1230),2);
    }
    // 1240
    #[test]
    fn t_1240_ju_test(){
        assert_eq!(test_ju_money_number(1240),0);
    }
    #[test]
    fn t_1240_goju_test(){
        assert_eq!(test_goju_money_number(1240),8);
    }
    #[test]
    fn t_1240_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1240),2);
    }
    #[test]
    fn t_1240_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1240),2);
    }
    // 1250
    #[test]
    fn t_1250_ju_test(){
        assert_eq!(test_ju_money_number(1250),0);
    }
    #[test]
    fn t_1250_goju_test(){
        assert_eq!(test_goju_money_number(1250),6);
    }
    #[test]
    fn t_1250_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1250),2);
    }
    #[test]
    fn t_1250_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1250),2);
    }
    // 1260
    #[test]
    fn t_1260_ju_test(){
        assert_eq!(test_ju_money_number(1260),30);
    }
    #[test]
    fn t_1260_goju_test(){
        assert_eq!(test_goju_money_number(1260),-2);
    }
    #[test]
    fn t_1260_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1260),2);
    }
    #[test]
    fn t_1260_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1260),2);
    }
    // 1270
    #[test]
    fn t_1270_ju_test(){
        assert_eq!(test_ju_money_number(1270),20);
    }
    #[test]
    fn t_1270_goju_test(){
        assert_eq!(test_goju_money_number(1270),-2);
    }
    #[test]
    fn t_1270_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1270),2);
    }
    #[test]
    fn t_1270_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1270),2);
    }
    // 1280
    #[test]
    fn t_1280_ju_test(){
        assert_eq!(test_ju_money_number(1280),10);
    }
    #[test]
    fn t_1280_goju_test(){
        assert_eq!(test_goju_money_number(1280),-2);
    }
    #[test]
    fn t_1280_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1280),2);
    }
    #[test]
    fn t_1280_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1280),2);
    }
    // 1290
    #[test]
    fn t_1290_ju_test(){
        assert_eq!(test_ju_money_number(1290),0);
    }
    #[test]
    fn t_1290_goju_test(){
        assert_eq!(test_goju_money_number(1290),-2);
    }
    #[test]
    fn t_1290_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1290),2);
    }
    #[test]
    fn t_1290_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1290),2);
    }
    // 1300
    #[test]
    fn t_1300_ju_test(){
        assert_eq!(test_ju_money_number(1300),0);
    }
    #[test]
    fn t_1300_goju_test(){
        assert_eq!(test_goju_money_number(1300),0);
    }
    #[test]
    fn t_1300_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1300),5);
    }
    #[test]
    fn t_1300_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1300),1);
    }
    // 1310
    #[test]
    fn t_1310_ju_test(){
        assert_eq!(test_ju_money_number(1310),30);
    }
    #[test]
    fn t_1310_goju_test(){
        assert_eq!(test_goju_money_number(1310),8);
    }
    #[test]
    fn t_1310_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1310),-8);
    }
    #[test]
    fn t_1310_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1310),2);
    }
    // 1320
    #[test]
    fn t_1320_ju_test(){
        assert_eq!(test_ju_money_number(1320),20);
    }
    #[test]
    fn t_1320_goju_test(){
        assert_eq!(test_goju_money_number(1320),8);
    }
    #[test]
    fn t_1320_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1320),-8);
    }
    #[test]
    fn t_1320_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1320),2);
    }
    // 1330
    #[test]
    fn t_1330_ju_test(){
        assert_eq!(test_ju_money_number(1330),10);
    }
    #[test]
    fn t_1330_goju_test(){
        assert_eq!(test_goju_money_number(1330),8);
    }
    #[test]
    fn t_1330_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1330),-8);
    }
    #[test]
    fn t_1330_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1330),2);
    }
    // 1340
    #[test]
    fn t_1340_ju_test(){
        assert_eq!(test_ju_money_number(1340),0);
    }
    #[test]
    fn t_1340_goju_test(){
        assert_eq!(test_goju_money_number(1340),8);
    }
    #[test]
    fn t_1340_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1340),-8);
    }
    #[test]
    fn t_1340_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1340),2);
    }
    // 1350
    #[test]
    fn t_1350_ju_test(){
        assert_eq!(test_ju_money_number(1350),0);
    }
    #[test]
    fn t_1350_goju_test(){
        assert_eq!(test_goju_money_number(1350),6);
    }
    #[test]
    fn t_1350_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1350),-8);
    }
    #[test]
    fn t_1350_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1350),2);
    }
    // 1360
    #[test]
    fn t_1360_ju_test(){
        assert_eq!(test_ju_money_number(1360),30);
    }
    #[test]
    fn t_1360_goju_test(){
        assert_eq!(test_goju_money_number(1360),-2);
    }
    #[test]
    fn t_1360_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1360),-8);
    }
    #[test]
    fn t_1360_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1360),2);
    }
    // 1370
    #[test]
    fn t_1370_ju_test(){
        assert_eq!(test_ju_money_number(1370),20);
    }
    #[test]
    fn t_1370_goju_test(){
        assert_eq!(test_goju_money_number(1370),-2);
    }
    #[test]
    fn t_1370_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1370),-8);
    }
    #[test]
    fn t_1370_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1370),2);
    }
    // 1380
    #[test]
    fn t_1380_ju_test(){
        assert_eq!(test_ju_money_number(1380),10);
    }
    #[test]
    fn t_1380_goju_test(){
        assert_eq!(test_goju_money_number(1380),-2);
    }
    #[test]
    fn t_1380_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1380),-8);
    }
    #[test]
    fn t_1380_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1380),2);
    }
    // 1390
    #[test]
    fn t_1390_ju_test(){
        assert_eq!(test_ju_money_number(1390),0);
    }
    #[test]
    fn t_1390_goju_test(){
        assert_eq!(test_goju_money_number(1390),-2);
    }
    #[test]
    fn t_1390_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1390),-8);
    }
    #[test]
    fn t_1390_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1390),2);
    }
    // 1400
    #[test]
    fn t_1400_ju_test(){
        assert_eq!(test_ju_money_number(1400),0);
    }
    #[test]
    fn t_1400_goju_test(){
        assert_eq!(test_goju_money_number(1400),0);
    }
    #[test]
    fn t_1400_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1400),-5);
    }
    #[test]
    fn t_1400_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1400),1);
    }
    // 1410
    #[test]
    fn t_1410_ju_test(){
        assert_eq!(test_ju_money_number(1410),25);
    }
    #[test]
    fn t_1410_goju_test(){
        assert_eq!(test_goju_money_number(1410),7);
    }
    #[test]
    fn t_1410_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1410),-12);
    }
    #[test]
    fn t_1410_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1410),1);
    }
    // 1420
    #[test]
    fn t_1420_ju_test(){
        assert_eq!(test_ju_money_number(1420),15);
    }
    #[test]
    fn t_1420_goju_test(){
        assert_eq!(test_goju_money_number(1420),7);
    }
    #[test]
    fn t_1420_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1420),-12);
    }
    #[test]
    fn t_1420_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1420),1);
    }
    // 1430
    #[test]
    fn t_1430_ju_test(){
        assert_eq!(test_ju_money_number(1430),5);
    }
    #[test]
    fn t_1430_goju_test(){
        assert_eq!(test_goju_money_number(1430),7);
    }
    #[test]
    fn t_1430_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1430),-12);
    }
    #[test]
    fn t_1430_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1430),1);
    }
    // 1440
    #[test]
    fn t_1440_ju_test(){
        assert_eq!(test_ju_money_number(1440),-5);
    }
    #[test]
    fn t_1440_goju_test(){
        assert_eq!(test_goju_money_number(1440),7);
    }
    #[test]
    fn t_1440_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1440),-12);
    }
    #[test]
    fn t_1440_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1440),1);
    }
    // 1450
    #[test]
    fn t_1450_ju_test(){
        assert_eq!(test_ju_money_number(1450),0);
    }
    #[test]
    fn t_1450_goju_test(){
        assert_eq!(test_goju_money_number(1450),4);
    }
    #[test]
    fn t_1450_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1450),-12);
    }
    #[test]
    fn t_1450_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1450),1);
    }
    // 1460
    #[test]
    fn t_1460_ju_test(){
        assert_eq!(test_ju_money_number(1460),25);
    }
    #[test]
    fn t_1460_goju_test(){
        assert_eq!(test_goju_money_number(1460),-3);
    }
    #[test]
    fn t_1460_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1460),-12);
    }
    #[test]
    fn t_1460_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1460),1);
    }
    // 1470
    #[test]
    fn t_1470_ju_test(){
        assert_eq!(test_ju_money_number(1470),15);
    }
    #[test]
    fn t_1470_goju_test(){
        assert_eq!(test_goju_money_number(1470),-3);
    }
    #[test]
    fn t_1470_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1470),-12);
    }
    #[test]
    fn t_1470_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1470),1);
    }
    // 1480
    #[test]
    fn t_1480_ju_test(){
        assert_eq!(test_ju_money_number(1480),5);
    }
    #[test]
    fn t_1480_goju_test(){
        assert_eq!(test_goju_money_number(1480),-3);
    }
    #[test]
    fn t_1480_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1480),-12);
    }
    #[test]
    fn t_1480_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1480),1);
    }
    // 1490
    #[test]
    fn t_1490_ju_test(){
        assert_eq!(test_ju_money_number(1490),-5);
    }
    #[test]
    fn t_1490_goju_test(){
        assert_eq!(test_goju_money_number(1490),-3);
    }
    #[test]
    fn t_1490_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1490),-12);
    }
    #[test]
    fn t_1490_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1490),1);
    }
    // 1500
    #[test]
    fn t_1500_ju_test(){
        assert_eq!(test_ju_money_number(1500),0);
    }
    #[test]
    fn t_1500_goju_test(){
        assert_eq!(test_goju_money_number(1500),0);
    }
    #[test]
    fn t_1500_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1500),0);
    }
    #[test]
    fn t_1500_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1500),0);
    }
    // 1510
    #[test]
    fn t_1510_ju_test(){
        assert_eq!(test_ju_money_number(1510),15);
    }
    #[test]
    fn t_1510_goju_test(){
        assert_eq!(test_goju_money_number(1510),5);
    }
    #[test]
    fn t_1510_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1510),20);
    }
    #[test]
    fn t_1510_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1510),-5);
    }
    // 1520
    #[test]
    fn t_1520_ju_test(){
        assert_eq!(test_ju_money_number(1520),5);
    }
    #[test]
    fn t_1520_goju_test(){
        assert_eq!(test_goju_money_number(1520),5);
    }
    #[test]
    fn t_1520_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1520),20);
    }
    #[test]
    fn t_1520_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1520),-5);
    }
    // 1530
    #[test]
    fn t_1530_ju_test(){
        assert_eq!(test_ju_money_number(1530),-5);
    }
    #[test]
    fn t_1530_goju_test(){
        assert_eq!(test_goju_money_number(1530),5);
    }
    #[test]
    fn t_1530_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1530),20);
    }
    #[test]
    fn t_1530_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1530),-5);
    }
    // 1540
    #[test]
    fn t_1540_ju_test(){
        assert_eq!(test_ju_money_number(1540),-15);
    }
    #[test]
    fn t_1540_goju_test(){
        assert_eq!(test_goju_money_number(1540),5);
    }
    #[test]
    fn t_1540_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1540),20);
    }
    #[test]
    fn t_1540_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1540),-5);
    }
    // 1550
    #[test]
    fn t_1550_ju_test(){
        assert_eq!(test_ju_money_number(1550),0);
    }
    #[test]
    fn t_1550_goju_test(){
        assert_eq!(test_goju_money_number(1550),0);
    }
    #[test]
    fn t_1550_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1550),20);
    }
    #[test]
    fn t_1550_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1550),-5);
    }
    // 1560
    #[test]
    fn t_1560_ju_test(){
        assert_eq!(test_ju_money_number(1560),15);
    }
    #[test]
    fn t_1560_goju_test(){
        assert_eq!(test_goju_money_number(1560),-5);
    }
    #[test]
    fn t_1560_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1560),20);
    }
    #[test]
    fn t_1560_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1560),-5);
    }
    // 1570
    #[test]
    fn t_1570_ju_test(){
        assert_eq!(test_ju_money_number(1570),5);
    }
    #[test]
    fn t_1570_goju_test(){
        assert_eq!(test_goju_money_number(1570),-5);
    }
    #[test]
    fn t_1570_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1570),20);
    }
    #[test]
    fn t_1570_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1570),-5);
    }
    // 1580
    #[test]
    fn t_1580_ju_test(){
        assert_eq!(test_ju_money_number(1580),-5);
    }
    #[test]
    fn t_1580_goju_test(){
        assert_eq!(test_goju_money_number(1580),-5);
    }
    #[test]
    fn t_1580_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1580),20);
    }
    #[test]
    fn t_1580_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1580),-5);
    }
    // 1590
    #[test]
    fn t_1590_ju_test(){
        assert_eq!(test_ju_money_number(1590),-15);
    }
    #[test]
    fn t_1590_goju_test(){
        assert_eq!(test_goju_money_number(1590),-5);
    }
    #[test]
    fn t_1590_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1590),20);
    }
    #[test]
    fn t_1590_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1590),-5);
    }
    // 1600
    #[test]
    fn t_1600_ju_test(){
        assert_eq!(test_ju_money_number(1600),0);
    }
    #[test]
    fn t_1600_goju_test(){
        assert_eq!(test_goju_money_number(1600),0);
    }
    #[test]
    fn t_1600_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1600),15);
    }
    #[test]
    fn t_1600_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1600),-5);
    }
    // 1610
    #[test]
    fn t_1610_ju_test(){
        assert_eq!(test_ju_money_number(1610),15);
    }
    #[test]
    fn t_1610_goju_test(){
        assert_eq!(test_goju_money_number(1610),5);
    }
    #[test]
    fn t_1610_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1610),10);
    }
    #[test]
    fn t_1610_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1610),-5);
    }
    // 1620
    #[test]
    fn t_1620_ju_test(){
        assert_eq!(test_ju_money_number(1620),5);
    }
    #[test]
    fn t_1620_goju_test(){
        assert_eq!(test_goju_money_number(1620),5);
    }
    #[test]
    fn t_1620_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1620),10);
    }
    #[test]
    fn t_1620_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1620),-5);
    }
    // 1630
    #[test]
    fn t_1630_ju_test(){
        assert_eq!(test_ju_money_number(1630),-5);
    }
    #[test]
    fn t_1630_goju_test(){
        assert_eq!(test_goju_money_number(1630),5);
    }
    #[test]
    fn t_1630_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1630),10);
    }
    #[test]
    fn t_1630_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1630),-5);
    }
    // 1640
    #[test]
    fn t_1640_ju_test(){
        assert_eq!(test_ju_money_number(1640),-15);
    }
    #[test]
    fn t_1640_goju_test(){
        assert_eq!(test_goju_money_number(1640),5);
    }
    #[test]
    fn t_1640_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1640),10);
    }
    #[test]
    fn t_1640_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1640),-5);
    }
    // 1650
    #[test]
    fn t_1650_ju_test(){
        assert_eq!(test_ju_money_number(1650),0);
    }
    #[test]
    fn t_1650_goju_test(){
        assert_eq!(test_goju_money_number(1650),0);
    }
    #[test]
    fn t_1650_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1650),10);
    }
    #[test]
    fn t_1650_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1650),-5);
    }
    // 1660
    #[test]
    fn t_1660_ju_test(){
        assert_eq!(test_ju_money_number(1660),15);
    }
    #[test]
    fn t_1660_goju_test(){
        assert_eq!(test_goju_money_number(1660),-5);
    }
    #[test]
    fn t_1660_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1660),10);
    }
    #[test]
    fn t_1660_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1660),-5);
    }
    // 1670
    #[test]
    fn t_1670_ju_test(){
        assert_eq!(test_ju_money_number(1670),5);
    }
    #[test]
    fn t_1670_goju_test(){
        assert_eq!(test_goju_money_number(1670),-5);
    }
    #[test]
    fn t_1670_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1670),10);
    }
    #[test]
    fn t_1670_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1670),-5);
    }
    // 1680
    #[test]
    fn t_1680_ju_test(){
        assert_eq!(test_ju_money_number(1680),-5);
    }
    #[test]
    fn t_1680_goju_test(){
        assert_eq!(test_goju_money_number(1680),-5);
    }
    #[test]
    fn t_1680_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1680),10);
    }
    #[test]
    fn t_1680_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1680),-5);
    }
    // 1690
    #[test]
    fn t_1690_ju_test(){
        assert_eq!(test_ju_money_number(1690),-15);
    }
    #[test]
    fn t_1690_goju_test(){
        assert_eq!(test_goju_money_number(1690),-5);
    }
    #[test]
    fn t_1690_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1690),10);
    }
    #[test]
    fn t_1690_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1690),-5);
    }
    // 1700
    #[test]
    fn t_1700_ju_test(){
        assert_eq!(test_ju_money_number(1700),0);
    }
    #[test]
    fn t_1700_goju_test(){
        assert_eq!(test_goju_money_number(1700),0);
    }
    #[test]
    fn t_1700_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1700),5);
    }
    #[test]
    fn t_1700_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1700),-5);
    }
    // 1710
    #[test]
    fn t_1710_ju_test(){
        assert_eq!(test_ju_money_number(1710),15);
    }
    #[test]
    fn t_1710_goju_test(){
        assert_eq!(test_goju_money_number(1710),5);
    }
    #[test]
    fn t_1710_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1710),0);
    }
    #[test]
    fn t_1710_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1710),-5);
    }
    // 1720
    #[test]
    fn t_1720_ju_test(){
        assert_eq!(test_ju_money_number(1720),5);
    }
    #[test]
    fn t_1720_goju_test(){
        assert_eq!(test_goju_money_number(1720),5);
    }
    #[test]
    fn t_1720_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1720),0);
    }
    #[test]
    fn t_1720_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1720),-5);
    }
    // 1730
    #[test]
    fn t_1730_ju_test(){
        assert_eq!(test_ju_money_number(1730),-5);
    }
    #[test]
    fn t_1730_goju_test(){
        assert_eq!(test_goju_money_number(1730),5);
    }
    #[test]
    fn t_1730_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1730),0);
    }
    #[test]
    fn t_1730_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1730),-5);
    }
    // 1740
    #[test]
    fn t_1740_ju_test(){
        assert_eq!(test_ju_money_number(1740),-15);
    }
    #[test]
    fn t_1740_goju_test(){
        assert_eq!(test_goju_money_number(1740),5);
    }
    #[test]
    fn t_1740_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1740),0);
    }
    #[test]
    fn t_1740_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1740),-5);
    }
    // 1750
    #[test]
    fn t_1750_ju_test(){
        assert_eq!(test_ju_money_number(1750),0);
    }
    #[test]
    fn t_1750_goju_test(){
        assert_eq!(test_goju_money_number(1750),0);
    }
    #[test]
    fn t_1750_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1750),0);
    }
    #[test]
    fn t_1750_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1750),-5);
    }
    // 1760
    #[test]
    fn t_1760_ju_test(){
        assert_eq!(test_ju_money_number(1760),15);
    }
    #[test]
    fn t_1760_goju_test(){
        assert_eq!(test_goju_money_number(1760),-5);
    }
    #[test]
    fn t_1760_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1760),0);
    }
    #[test]
    fn t_1760_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1760),-5);
    }
    // 1770
    #[test]
    fn t_1770_ju_test(){
        assert_eq!(test_ju_money_number(1770),5);
    }
    #[test]
    fn t_1770_goju_test(){
        assert_eq!(test_goju_money_number(1770),-5);
    }
    #[test]
    fn t_1770_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1770),0);
    }
    #[test]
    fn t_1770_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1770),-5);
    }
    // 1780
    #[test]
    fn t_1780_ju_test(){
        assert_eq!(test_ju_money_number(1780),-5);
    }
    #[test]
    fn t_1780_goju_test(){
        assert_eq!(test_goju_money_number(1780),-5);
    }
    #[test]
    fn t_1780_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1780),0);
    }
    #[test]
    fn t_1780_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1780),-5);
    }
    // 1790
    #[test]
    fn t_1790_ju_test(){
        assert_eq!(test_ju_money_number(1790),-15);
    }
    #[test]
    fn t_1790_goju_test(){
        assert_eq!(test_goju_money_number(1790),-5);
    }
    #[test]
    fn t_1790_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1790),0);
    }
    #[test]
    fn t_1790_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1790),-5);
    }
    // 1800
    #[test]
    fn t_1800_ju_test(){
        assert_eq!(test_ju_money_number(1800),0);
    }
    #[test]
    fn t_1800_goju_test(){
        assert_eq!(test_goju_money_number(1800),0);
    }
    #[test]
    fn t_1800_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1800),-5);
    }
    #[test]
    fn t_1800_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1800),-5);
    }
    // 1810
    #[test]
    fn t_1810_ju_test(){
        assert_eq!(test_ju_money_number(1810),15);
    }
    #[test]
    fn t_1810_goju_test(){
        assert_eq!(test_goju_money_number(1810),5);
    }
    #[test]
    fn t_1810_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1810),-10);
    }
    #[test]
    fn t_1810_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1810),-5);
    }
    // 1820
    #[test]
    fn t_1820_ju_test(){
        assert_eq!(test_ju_money_number(1820),5);
    }
    #[test]
    fn t_1820_goju_test(){
        assert_eq!(test_goju_money_number(1820),5);
    }
    #[test]
    fn t_1820_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1820),-10);
    }
    #[test]
    fn t_1820_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1820),-5);
    }
    // 1830
    #[test]
    fn t_1830_ju_test(){
        assert_eq!(test_ju_money_number(1830),-5);
    }
    #[test]
    fn t_1830_goju_test(){
        assert_eq!(test_goju_money_number(1830),5);
    }
    #[test]
    fn t_1830_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1830),-10);
    }
    #[test]
    fn t_1830_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1830),-5);
    }
    // 1840
    #[test]
    fn t_1840_ju_test(){
        assert_eq!(test_ju_money_number(1840),-15);
    }
    #[test]
    fn t_1840_goju_test(){
        assert_eq!(test_goju_money_number(1840),5);
    }
    #[test]
    fn t_1840_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1840),-10);
    }
    #[test]
    fn t_1840_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1840),-5);
    }
    // 1850
    #[test]
    fn t_1850_ju_test(){
        assert_eq!(test_ju_money_number(1850),0);
    }
    #[test]
    fn t_1850_goju_test(){
        assert_eq!(test_goju_money_number(1850),0);
    }
    #[test]
    fn t_1850_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1850),-10);
    }
    #[test]
    fn t_1850_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1850),-5);
    }
    // 1860
    #[test]
    fn t_1860_ju_test(){
        assert_eq!(test_ju_money_number(1860),15);
    }
    #[test]
    fn t_1860_goju_test(){
        assert_eq!(test_goju_money_number(1860),-5);
    }
    #[test]
    fn t_1860_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1860),-10);
    }
    #[test]
    fn t_1860_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1860),-5);
    }
    // 1870
    #[test]
    fn t_1870_ju_test(){
        assert_eq!(test_ju_money_number(1870),5);
    }
    #[test]
    fn t_1870_goju_test(){
        assert_eq!(test_goju_money_number(1870),-5);
    }
    #[test]
    fn t_1870_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1870),-10);
    }
    #[test]
    fn t_1870_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1870),-5);
    }
    // 1880
    #[test]
    fn t_1880_ju_test(){
        assert_eq!(test_ju_money_number(1880),-5);
    }
    #[test]
    fn t_1880_goju_test(){
        assert_eq!(test_goju_money_number(1880),-5);
    }
    #[test]
    fn t_1880_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1880),-10);
    }
    #[test]
    fn t_1880_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1880),-5);
    }
    // 1890
    #[test]
    fn t_1890_ju_test(){
        assert_eq!(test_ju_money_number(1890),-15);
    }
    #[test]
    fn t_1890_goju_test(){
        assert_eq!(test_goju_money_number(1890),-5);
    }
    #[test]
    fn t_1890_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1890),-10);
    }
    #[test]
    fn t_1890_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1890),-5);
    }
    // 1900
    #[test]
    fn t_1900_ju_test(){
        assert_eq!(test_ju_money_number(1900),0);
    }
    #[test]
    fn t_1900_goju_test(){
        assert_eq!(test_goju_money_number(1900),0);
    }
    #[test]
    fn t_1900_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1900),-15);
    }
    #[test]
    fn t_1900_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1900),-5);
    }
    // 1910
    #[test]
    fn t_1910_ju_test(){
        assert_eq!(test_ju_money_number(1910),15);
    }
    #[test]
    fn t_1910_goju_test(){
        assert_eq!(test_goju_money_number(1910),5);
    }
    #[test]
    fn t_1910_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1910),-20);
    }
    #[test]
    fn t_1910_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1910),-5);
    }
    // 1920
    #[test]
    fn t_1920_ju_test(){
        assert_eq!(test_ju_money_number(1920),5);
    }
    #[test]
    fn t_1920_goju_test(){
        assert_eq!(test_goju_money_number(1920),5);
    }
    #[test]
    fn t_1920_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1920),-20);
    }
    #[test]
    fn t_1920_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1920),-5);
    }
    // 1930
    #[test]
    fn t_1930_ju_test(){
        assert_eq!(test_ju_money_number(1930),-5);
    }
    #[test]
    fn t_1930_goju_test(){
        assert_eq!(test_goju_money_number(1930),5);
    }
    #[test]
    fn t_1930_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1930),-20);
    }
    #[test]
    fn t_1930_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1930),-5);
    }
    // 1940
    #[test]
    fn t_1940_ju_test(){
        assert_eq!(test_ju_money_number(1940),-15);
    }
    #[test]
    fn t_1940_goju_test(){
        assert_eq!(test_goju_money_number(1940),5);
    }
    #[test]
    fn t_1940_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1940),-20);
    }
    #[test]
    fn t_1940_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1940),-5);
    }
    // 1950
    #[test]
    fn t_1950_ju_test(){
        assert_eq!(test_ju_money_number(1950),0);
    }
    #[test]
    fn t_1950_goju_test(){
        assert_eq!(test_goju_money_number(1950),0);
    }
    #[test]
    fn t_1950_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1950),-20);
    }
    #[test]
    fn t_1950_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1950),-5);
    }
    // 1960
    #[test]
    fn t_1960_ju_test(){
        assert_eq!(test_ju_money_number(1960),15);
    }
    #[test]
    fn t_1960_goju_test(){
        assert_eq!(test_goju_money_number(1960),-5);
    }
    #[test]
    fn t_1960_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1960),-20);
    }
    #[test]
    fn t_1960_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1960),-5);
    }
    // 1970
    #[test]
    fn t_1970_ju_test(){
        assert_eq!(test_ju_money_number(1970),5);
    }
    #[test]
    fn t_1970_goju_test(){
        assert_eq!(test_goju_money_number(1970),-5);
    }
    #[test]
    fn t_1970_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1970),-20);
    }
    #[test]
    fn t_1970_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1970),-5);
    }
    // 1980
    #[test]
    fn t_1980_ju_test(){
        assert_eq!(test_ju_money_number(1980),-5);
    }
    #[test]
    fn t_1980_goju_test(){
        assert_eq!(test_goju_money_number(1980),-5);
    }
    #[test]
    fn t_1980_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1980),-20);
    }
    #[test]
    fn t_1980_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1980),-5);
    }
    // 1990
    #[test]
    fn t_1990_ju_test(){
        assert_eq!(test_ju_money_number(1990),-15);
    }
    #[test]
    fn t_1990_goju_test(){
        assert_eq!(test_goju_money_number(1990),-5);
    }
    #[test]
    fn t_1990_hyaku_test(){
        assert_eq!(test_hyaku_money_number(1990),-20);
    }
    #[test]
    fn t_1990_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(1990),-5);
    }
    // 2000
    #[test]
    fn t_2000_ju_test(){
        assert_eq!(test_ju_money_number(2000),0);
    }
    #[test]
    fn t_2000_goju_test(){
        assert_eq!(test_goju_money_number(2000),0);
    }
    #[test]
    fn t_2000_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2000),0);
    }
    #[test]
    fn t_2000_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2000),0);
    }
    // 2010
    #[test]
    fn t_2010_ju_test(){
        assert_eq!(test_ju_money_number(2010),25);
    }
    #[test]
    fn t_2010_goju_test(){
        assert_eq!(test_goju_money_number(2010),1);
    }
    #[test]
    fn t_2010_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2010),-4);
    }
    #[test]
    fn t_2010_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2010),0);
    }
    // 2020
    #[test]
    fn t_2020_ju_test(){
        assert_eq!(test_ju_money_number(2020),15);
    }
    #[test]
    fn t_2020_goju_test(){
        assert_eq!(test_goju_money_number(2020),1);
    }
    #[test]
    fn t_2020_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2020),-4);
    }
    #[test]
    fn t_2020_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2020),0);
    }
    // 2030
    #[test]
    fn t_2030_ju_test(){
        assert_eq!(test_ju_money_number(2030),5);
    }
    #[test]
    fn t_2030_goju_test(){
        assert_eq!(test_goju_money_number(2030),1);
    }
    #[test]
    fn t_2030_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2030),-4);
    }
    #[test]
    fn t_2030_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2030),0);
    }
    // 2040
    #[test]
    fn t_2040_ju_test(){
        assert_eq!(test_ju_money_number(2040),-5);
    }
    #[test]
    fn t_2040_goju_test(){
        assert_eq!(test_goju_money_number(2040),1);
    }
    #[test]
    fn t_2040_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2040),-4);
    }
    #[test]
    fn t_2040_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2040),0);
    }
    // 2050
    #[test]
    fn t_2050_ju_test(){
        assert_eq!(test_ju_money_number(2050),0);
    }
    #[test]
    fn t_2050_goju_test(){
        assert_eq!(test_goju_money_number(2050),0);
    }
    #[test]
    fn t_2050_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2050),-5);
    }
    #[test]
    fn t_2050_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2050),0);
    }
    // 2060
    #[test]
    fn t_2060_ju_test(){
        assert_eq!(test_ju_money_number(2060),15);
    }
    #[test]
    fn t_2060_goju_test(){
        assert_eq!(test_goju_money_number(2060),-5);
    }
    #[test]
    fn t_2060_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2060),-5);
    }
    #[test]
    fn t_2060_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2060),0);
    }
    // 2070
    #[test]
    fn t_2070_ju_test(){
        assert_eq!(test_ju_money_number(2070),5);
    }
    #[test]
    fn t_2070_goju_test(){
        assert_eq!(test_goju_money_number(2070),-5);
    }
    #[test]
    fn t_2070_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2070),-5);
    }
    #[test]
    fn t_2070_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2070),0);
    }
    // 2080
    #[test]
    fn t_2080_ju_test(){
        assert_eq!(test_ju_money_number(2080),-5);
    }
    #[test]
    fn t_2080_goju_test(){
        assert_eq!(test_goju_money_number(2080),-5);
    }
    #[test]
    fn t_2080_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2080),-5);
    }
    #[test]
    fn t_2080_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2080),0);
    }
    // 2090
    #[test]
    fn t_2090_ju_test(){
        assert_eq!(test_ju_money_number(2090),-15);
    }
    #[test]
    fn t_2090_goju_test(){
        assert_eq!(test_goju_money_number(2090),-5);
    }
    #[test]
    fn t_2090_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2090),-5);
    }
    #[test]
    fn t_2090_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2090),0);
    }
    // 2110
    #[test]
    fn t_2110_ju_test(){
        assert_eq!(test_ju_money_number(2110),30);
    }
    #[test]
    fn t_2110_goju_test(){
        assert_eq!(test_goju_money_number(2110),8);
    }
    #[test]
    fn t_2110_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2110),12);
    }
    #[test]
    fn t_2110_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2110),2);
    }
    // 2120
    #[test]
    fn t_2120_ju_test(){
        assert_eq!(test_ju_money_number(2120),20);
    }
    #[test]
    fn t_2120_goju_test(){
        assert_eq!(test_goju_money_number(2120),8);
    }
    #[test]
    fn t_2120_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2120),12);
    }
    #[test]
    fn t_2120_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2120),2);
    }
    // 2130
    #[test]
    fn t_2130_ju_test(){
        assert_eq!(test_ju_money_number(2130),10);
    }
    #[test]
    fn t_2130_goju_test(){
        assert_eq!(test_goju_money_number(2130),8);
    }
    #[test]
    fn t_2130_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2130),12);
    }
    #[test]
    fn t_2130_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2130),2);
    }
    // 2140
    #[test]
    fn t_2140_ju_test(){
        assert_eq!(test_ju_money_number(2140),0);
    }
    #[test]
    fn t_2140_goju_test(){
        assert_eq!(test_goju_money_number(2140),8);
    }
    #[test]
    fn t_2140_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2140),12);
    }
    #[test]
    fn t_2140_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2140),2);
    }
    // 2150
    #[test]
    fn t_2150_ju_test(){
        assert_eq!(test_ju_money_number(2150),0);
    }
    #[test]
    fn t_2150_goju_test(){
        assert_eq!(test_goju_money_number(2150),6);
    }
    #[test]
    fn t_2150_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2150),12);
    }
    #[test]
    fn t_2150_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2150),2);
    }
    // 2160
    #[test]
    fn t_2160_ju_test(){
        assert_eq!(test_ju_money_number(2160),30);
    }
    #[test]
    fn t_2160_goju_test(){
        assert_eq!(test_goju_money_number(2160),-2);
    }
    #[test]
    fn t_2160_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2160),12);
    }
    #[test]
    fn t_2160_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2160),2);
    }
    // 2170
    #[test]
    fn t_2170_ju_test(){
        assert_eq!(test_ju_money_number(2170),20);
    }
    #[test]
    fn t_2170_goju_test(){
        assert_eq!(test_goju_money_number(2170),-2);
    }
    #[test]
    fn t_2170_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2170),12);
    }
    #[test]
    fn t_2170_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2170),2);
    }
    // 2180
    #[test]
    fn t_2180_ju_test(){
        assert_eq!(test_ju_money_number(2180),10);
    }
    #[test]
    fn t_2180_goju_test(){
        assert_eq!(test_goju_money_number(2180),-2);
    }
    #[test]
    fn t_2180_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2180),12);
    }
    #[test]
    fn t_2180_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2180),2);
    }
    // 2190
    #[test]
    fn t_2190_ju_test(){
        assert_eq!(test_ju_money_number(2190),0);
    }
    #[test]
    fn t_2190_goju_test(){
        assert_eq!(test_goju_money_number(2190),-2);
    }
    #[test]
    fn t_2190_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2190),12);
    }
    #[test]
    fn t_2190_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2190),2);
    }
    // 2200
    #[test]
    fn t_2200_ju_test(){
        assert_eq!(test_ju_money_number(2200),0);
    }
    #[test]
    fn t_2200_goju_test(){
        assert_eq!(test_goju_money_number(2200),0);
    }
    #[test]
    fn t_2200_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2200),15);
    }
    #[test]
    fn t_2200_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2200),1);
    }
    // 2210
    #[test]
    fn t_2210_ju_test(){
        assert_eq!(test_ju_money_number(2210),30);
    }
    #[test]
    fn t_2210_goju_test(){
        assert_eq!(test_goju_money_number(2210),8);
    }
    #[test]
    fn t_2210_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2210),2);
    }
    #[test]
    fn t_2210_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2210),2);
    }
    // 2220
    #[test]
    fn t_2220_ju_test(){
        assert_eq!(test_ju_money_number(2220),20);
    }
    #[test]
    fn t_2220_goju_test(){
        assert_eq!(test_goju_money_number(2220),8);
    }
    #[test]
    fn t_2220_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2220),2);
    }
    #[test]
    fn t_2220_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2220),2);
    }
    // 2230
    #[test]
    fn t_2230_ju_test(){
        assert_eq!(test_ju_money_number(2230),10);
    }
    #[test]
    fn t_2230_goju_test(){
        assert_eq!(test_goju_money_number(2230),8);
    }
    #[test]
    fn t_2230_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2230),2);
    }
    #[test]
    fn t_2230_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2230),2);
    }
    // 2240
    #[test]
    fn t_2240_ju_test(){
        assert_eq!(test_ju_money_number(2240),0);
    }
    #[test]
    fn t_2240_goju_test(){
        assert_eq!(test_goju_money_number(2240),8);
    }
    #[test]
    fn t_2240_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2240),2);
    }
    #[test]
    fn t_2240_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2240),2);
    }
    // 2250
    #[test]
    fn t_2250_ju_test(){
        assert_eq!(test_ju_money_number(2250),0);
    }
    #[test]
    fn t_2250_goju_test(){
        assert_eq!(test_goju_money_number(2250),6);
    }
    #[test]
    fn t_2250_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2250),2);
    }
    #[test]
    fn t_2250_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2250),2);
    }
    // 2260
    #[test]
    fn t_2260_ju_test(){
        assert_eq!(test_ju_money_number(2260),30);
    }
    #[test]
    fn t_2260_goju_test(){
        assert_eq!(test_goju_money_number(2260),-2);
    }
    #[test]
    fn t_2260_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2260),2);
    }
    #[test]
    fn t_2260_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2260),2);
    }
    // 2270
    #[test]
    fn t_2270_ju_test(){
        assert_eq!(test_ju_money_number(2270),20);
    }
    #[test]
    fn t_2270_goju_test(){
        assert_eq!(test_goju_money_number(2270),-2);
    }
    #[test]
    fn t_2270_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2270),2);
    }
    #[test]
    fn t_2270_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2270),2);
    }
    // 2280
    #[test]
    fn t_2280_ju_test(){
        assert_eq!(test_ju_money_number(2280),10);
    }
    #[test]
    fn t_2280_goju_test(){
        assert_eq!(test_goju_money_number(2280),-2);
    }
    #[test]
    fn t_2280_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2280),2);
    }
    #[test]
    fn t_2280_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2280),2);
    }
    // 2290
    #[test]
    fn t_2290_ju_test(){
        assert_eq!(test_ju_money_number(2290),0);
    }
    #[test]
    fn t_2290_goju_test(){
        assert_eq!(test_goju_money_number(2290),-2);
    }
    #[test]
    fn t_2290_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2290),2);
    }
    #[test]
    fn t_2290_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2290),2);
    }
    // 2300
    #[test]
    fn t_2300_ju_test(){
        assert_eq!(test_ju_money_number(2300),0);
    }
    #[test]
    fn t_2300_goju_test(){
        assert_eq!(test_goju_money_number(2300),0);
    }
    #[test]
    fn t_2300_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2300),5);
    }
    #[test]
    fn t_2300_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2300),1);
    }
    // 2310
    #[test]
    fn t_2310_ju_test(){
        assert_eq!(test_ju_money_number(2310),30);
    }
    #[test]
    fn t_2310_goju_test(){
        assert_eq!(test_goju_money_number(2310),8);
    }
    #[test]
    fn t_2310_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2310),-8);
    }
    #[test]
    fn t_2310_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2310),2);
    }
    // 2320
    #[test]
    fn t_2320_ju_test(){
        assert_eq!(test_ju_money_number(2320),20);
    }
    #[test]
    fn t_2320_goju_test(){
        assert_eq!(test_goju_money_number(2320),8);
    }
    #[test]
    fn t_2320_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2320),-8);
    }
    #[test]
    fn t_2320_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2320),2);
    }
    // 2330
    #[test]
    fn t_2330_ju_test(){
        assert_eq!(test_ju_money_number(2330),10);
    }
    #[test]
    fn t_2330_goju_test(){
        assert_eq!(test_goju_money_number(2330),8);
    }
    #[test]
    fn t_2330_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2330),-8);
    }
    #[test]
    fn t_2330_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2330),2);
    }
    // 2340
    #[test]
    fn t_2340_ju_test(){
        assert_eq!(test_ju_money_number(2340),0);
    }
    #[test]
    fn t_2340_goju_test(){
        assert_eq!(test_goju_money_number(2340),8);
    }
    #[test]
    fn t_2340_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2340),-8);
    }
    #[test]
    fn t_2340_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2340),2);
    }
    // 2350
    #[test]
    fn t_2350_ju_test(){
        assert_eq!(test_ju_money_number(2350),0);
    }
    #[test]
    fn t_2350_goju_test(){
        assert_eq!(test_goju_money_number(2350),6);
    }
    #[test]
    fn t_2350_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2350),-8);
    }
    #[test]
    fn t_2350_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2350),2);
    }
    // 2360
    #[test]
    fn t_2360_ju_test(){
        assert_eq!(test_ju_money_number(2360),30);
    }
    #[test]
    fn t_2360_goju_test(){
        assert_eq!(test_goju_money_number(2360),-2);
    }
    #[test]
    fn t_2360_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2360),-8);
    }
    #[test]
    fn t_2360_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2360),2);
    }
    // 2370
    #[test]
    fn t_2370_ju_test(){
        assert_eq!(test_ju_money_number(2370),20);
    }
    #[test]
    fn t_2370_goju_test(){
        assert_eq!(test_goju_money_number(2370),-2);
    }
    #[test]
    fn t_2370_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2370),-8);
    }
    #[test]
    fn t_2370_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2370),2);
    }
    // 2380
    #[test]
    fn t_2380_ju_test(){
        assert_eq!(test_ju_money_number(2380),10);
    }
    #[test]
    fn t_2380_goju_test(){
        assert_eq!(test_goju_money_number(2380),-2);
    }
    #[test]
    fn t_2380_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2380),-8);
    }
    #[test]
    fn t_2380_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2380),2);
    }
    // 2390
    #[test]
    fn t_2390_ju_test(){
        assert_eq!(test_ju_money_number(2390),0);
    }
    #[test]
    fn t_2390_goju_test(){
        assert_eq!(test_goju_money_number(2390),-2);
    }
    #[test]
    fn t_2390_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2390),-8);
    }
    #[test]
    fn t_2390_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2390),2);
    }
    // 2400
    #[test]
    fn t_2400_ju_test(){
        assert_eq!(test_ju_money_number(2400),0);
    }
    #[test]
    fn t_2400_goju_test(){
        assert_eq!(test_goju_money_number(2400),0);
    }
    #[test]
    fn t_2400_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2400),-5);
    }
    #[test]
    fn t_2400_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2400),1);
    }
    // 2410
    #[test]
    fn t_2410_ju_test(){
        assert_eq!(test_ju_money_number(2410),25);
    }
    #[test]
    fn t_2410_goju_test(){
        assert_eq!(test_goju_money_number(2410),7);
    }
    #[test]
    fn t_2410_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2410),-12);
    }
    #[test]
    fn t_2410_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2410),1);
    }
    // 2420
    #[test]
    fn t_2420_ju_test(){
        assert_eq!(test_ju_money_number(2420),15);
    }
    #[test]
    fn t_2420_goju_test(){
        assert_eq!(test_goju_money_number(2420),7);
    }
    #[test]
    fn t_2420_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2420),-12);
    }
    #[test]
    fn t_2420_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2420),1);
    }
    // 2430
    #[test]
    fn t_2430_ju_test(){
        assert_eq!(test_ju_money_number(2430),5);
    }
    #[test]
    fn t_2430_goju_test(){
        assert_eq!(test_goju_money_number(2430),7);
    }
    #[test]
    fn t_2430_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2430),-12);
    }
    #[test]
    fn t_2430_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2430),1);
    }
    // 2440
    #[test]
    fn t_2440_ju_test(){
        assert_eq!(test_ju_money_number(2440),-5);
    }
    #[test]
    fn t_2440_goju_test(){
        assert_eq!(test_goju_money_number(2440),7);
    }
    #[test]
    fn t_2440_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2440),-12);
    }
    #[test]
    fn t_2440_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2440),1);
    }
    // 2450
    #[test]
    fn t_2450_ju_test(){
        assert_eq!(test_ju_money_number(2450),0);
    }
    #[test]
    fn t_2450_goju_test(){
        assert_eq!(test_goju_money_number(2450),4);
    }
    #[test]
    fn t_2450_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2450),-12);
    }
    #[test]
    fn t_2450_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2450),1);
    }
    // 2460
    #[test]
    fn t_2460_ju_test(){
        assert_eq!(test_ju_money_number(2460),25);
    }
    #[test]
    fn t_2460_goju_test(){
        assert_eq!(test_goju_money_number(2460),-3);
    }
    #[test]
    fn t_2460_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2460),-12);
    }
    #[test]
    fn t_2460_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2460),1);
    }
    // 2470
    #[test]
    fn t_2470_ju_test(){
        assert_eq!(test_ju_money_number(2470),15);
    }
    #[test]
    fn t_2470_goju_test(){
        assert_eq!(test_goju_money_number(2470),-3);
    }
    #[test]
    fn t_2470_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2470),-12);
    }
    #[test]
    fn t_2470_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2470),1);
    }
    // 2480
    #[test]
    fn t_2480_ju_test(){
        assert_eq!(test_ju_money_number(2480),5);
    }
    #[test]
    fn t_2480_goju_test(){
        assert_eq!(test_goju_money_number(2480),-3);
    }
    #[test]
    fn t_2480_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2480),-12);
    }
    #[test]
    fn t_2480_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2480),1);
    }
    // 2490
    #[test]
    fn t_2490_ju_test(){
        assert_eq!(test_ju_money_number(2490),-5);
    }
    #[test]
    fn t_2490_goju_test(){
        assert_eq!(test_goju_money_number(2490),-3);
    }
    #[test]
    fn t_2490_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2490),-12);
    }
    #[test]
    fn t_2490_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2490),1);
    }
    // 2500
    #[test]
    fn t_2500_ju_test(){
        assert_eq!(test_ju_money_number(2500),0);
    }
    #[test]
    fn t_2500_goju_test(){
        assert_eq!(test_goju_money_number(2500),0);
    }
    #[test]
    fn t_2500_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2500),0);
    }
    #[test]
    fn t_2500_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2500),0);
    }
    // 2510
    #[test]
    fn t_2510_ju_test(){
        assert_eq!(test_ju_money_number(2510),15);
    }
    #[test]
    fn t_2510_goju_test(){
        assert_eq!(test_goju_money_number(2510),5);
    }
    #[test]
    fn t_2510_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2510),20);
    }
    #[test]
    fn t_2510_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2510),-5);
    }
    // 2520
    #[test]
    fn t_2520_ju_test(){
        assert_eq!(test_ju_money_number(2520),5);
    }
    #[test]
    fn t_2520_goju_test(){
        assert_eq!(test_goju_money_number(2520),5);
    }
    #[test]
    fn t_2520_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2520),20);
    }
    #[test]
    fn t_2520_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2520),-5);
    }
    // 2530
    #[test]
    fn t_2530_ju_test(){
        assert_eq!(test_ju_money_number(2530),-5);
    }
    #[test]
    fn t_2530_goju_test(){
        assert_eq!(test_goju_money_number(2530),5);
    }
    #[test]
    fn t_2530_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2530),20);
    }
    #[test]
    fn t_2530_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2530),-5);
    }
    // 2540
    #[test]
    fn t_2540_ju_test(){
        assert_eq!(test_ju_money_number(2540),-15);
    }
    #[test]
    fn t_2540_goju_test(){
        assert_eq!(test_goju_money_number(2540),5);
    }
    #[test]
    fn t_2540_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2540),20);
    }
    #[test]
    fn t_2540_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2540),-5);
    }
    // 2550
    #[test]
    fn t_2550_ju_test(){
        assert_eq!(test_ju_money_number(2550),0);
    }
    #[test]
    fn t_2550_goju_test(){
        assert_eq!(test_goju_money_number(2550),0);
    }
    #[test]
    fn t_2550_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2550),20);
    }
    #[test]
    fn t_2550_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2550),-5);
    }
    // 2560
    #[test]
    fn t_2560_ju_test(){
        assert_eq!(test_ju_money_number(2560),15);
    }
    #[test]
    fn t_2560_goju_test(){
        assert_eq!(test_goju_money_number(2560),-5);
    }
    #[test]
    fn t_2560_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2560),20);
    }
    #[test]
    fn t_2560_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2560),-5);
    }
    // 2570
    #[test]
    fn t_2570_ju_test(){
        assert_eq!(test_ju_money_number(2570),5);
    }
    #[test]
    fn t_2570_goju_test(){
        assert_eq!(test_goju_money_number(2570),-5);
    }
    #[test]
    fn t_2570_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2570),20);
    }
    #[test]
    fn t_2570_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2570),-5);
    }
    // 2580
    #[test]
    fn t_2580_ju_test(){
        assert_eq!(test_ju_money_number(2580),-5);
    }
    #[test]
    fn t_2580_goju_test(){
        assert_eq!(test_goju_money_number(2580),-5);
    }
    #[test]
    fn t_2580_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2580),20);
    }
    #[test]
    fn t_2580_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2580),-5);
    }
    // 2590
    #[test]
    fn t_2590_ju_test(){
        assert_eq!(test_ju_money_number(2590),-15);
    }
    #[test]
    fn t_2590_goju_test(){
        assert_eq!(test_goju_money_number(2590),-5);
    }
    #[test]
    fn t_2590_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2590),20);
    }
    #[test]
    fn t_2590_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2590),-5);
    }
    // 2600
    #[test]
    fn t_2600_ju_test(){
        assert_eq!(test_ju_money_number(2600),0);
    }
    #[test]
    fn t_2600_goju_test(){
        assert_eq!(test_goju_money_number(2600),0);
    }
    #[test]
    fn t_2600_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2600),15);
    }
    #[test]
    fn t_2600_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2600),-5);
    }
    // 2610
    #[test]
    fn t_2610_ju_test(){
        assert_eq!(test_ju_money_number(2610),15);
    }
    #[test]
    fn t_2610_goju_test(){
        assert_eq!(test_goju_money_number(2610),5);
    }
    #[test]
    fn t_2610_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2610),10);
    }
    #[test]
    fn t_2610_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2610),-5);
    }
    // 2620
    #[test]
    fn t_2620_ju_test(){
        assert_eq!(test_ju_money_number(2620),5);
    }
    #[test]
    fn t_2620_goju_test(){
        assert_eq!(test_goju_money_number(2620),5);
    }
    #[test]
    fn t_2620_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2620),10);
    }
    #[test]
    fn t_2620_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2620),-5);
    }
    // 2630
    #[test]
    fn t_2630_ju_test(){
        assert_eq!(test_ju_money_number(2630),-5);
    }
    #[test]
    fn t_2630_goju_test(){
        assert_eq!(test_goju_money_number(2630),5);
    }
    #[test]
    fn t_2630_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2630),10);
    }
    #[test]
    fn t_2630_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2630),-5);
    }
    // 2640
    #[test]
    fn t_2640_ju_test(){
        assert_eq!(test_ju_money_number(2640),-15);
    }
    #[test]
    fn t_2640_goju_test(){
        assert_eq!(test_goju_money_number(2640),5);
    }
    #[test]
    fn t_2640_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2640),10);
    }
    #[test]
    fn t_2640_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2640),-5);
    }
    // 2650
    #[test]
    fn t_2650_ju_test(){
        assert_eq!(test_ju_money_number(2650),0);
    }
    #[test]
    fn t_2650_goju_test(){
        assert_eq!(test_goju_money_number(2650),0);
    }
    #[test]
    fn t_2650_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2650),10);
    }
    #[test]
    fn t_2650_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2650),-5);
    }
    // 2660
    #[test]
    fn t_2660_ju_test(){
        assert_eq!(test_ju_money_number(2660),15);
    }
    #[test]
    fn t_2660_goju_test(){
        assert_eq!(test_goju_money_number(2660),-5);
    }
    #[test]
    fn t_2660_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2660),10);
    }
    #[test]
    fn t_2660_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2660),-5);
    }
    // 2670
    #[test]
    fn t_2670_ju_test(){
        assert_eq!(test_ju_money_number(2670),5);
    }
    #[test]
    fn t_2670_goju_test(){
        assert_eq!(test_goju_money_number(2670),-5);
    }
    #[test]
    fn t_2670_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2670),10);
    }
    #[test]
    fn t_2670_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2670),-5);
    }
    // 2680
    #[test]
    fn t_2680_ju_test(){
        assert_eq!(test_ju_money_number(2680),-5);
    }
    #[test]
    fn t_2680_goju_test(){
        assert_eq!(test_goju_money_number(2680),-5);
    }
    #[test]
    fn t_2680_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2680),10);
    }
    #[test]
    fn t_2680_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2680),-5);
    }
    // 2690
    #[test]
    fn t_2690_ju_test(){
        assert_eq!(test_ju_money_number(2690),-15);
    }
    #[test]
    fn t_2690_goju_test(){
        assert_eq!(test_goju_money_number(2690),-5);
    }
    #[test]
    fn t_2690_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2690),10);
    }
    #[test]
    fn t_2690_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2690),-5);
    }
    // 2700
    #[test]
    fn t_2700_ju_test(){
        assert_eq!(test_ju_money_number(2700),0);
    }
    #[test]
    fn t_2700_goju_test(){
        assert_eq!(test_goju_money_number(2700),0);
    }
    #[test]
    fn t_2700_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2700),5);
    }
    #[test]
    fn t_2700_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2700),-5);
    }
    // 2710
    #[test]
    fn t_2710_ju_test(){
        assert_eq!(test_ju_money_number(2710),15);
    }
    #[test]
    fn t_2710_goju_test(){
        assert_eq!(test_goju_money_number(2710),5);
    }
    #[test]
    fn t_2710_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2710),0);
    }
    #[test]
    fn t_2710_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2710),-5);
    }
    // 2720
    #[test]
    fn t_2720_ju_test(){
        assert_eq!(test_ju_money_number(2720),5);
    }
    #[test]
    fn t_2720_goju_test(){
        assert_eq!(test_goju_money_number(2720),5);
    }
    #[test]
    fn t_2720_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2720),0);
    }
    #[test]
    fn t_2720_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2720),-5);
    }
    // 2730
    #[test]
    fn t_2730_ju_test(){
        assert_eq!(test_ju_money_number(2730),-5);
    }
    #[test]
    fn t_2730_goju_test(){
        assert_eq!(test_goju_money_number(2730),5);
    }
    #[test]
    fn t_2730_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2730),0);
    }
    #[test]
    fn t_2730_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2730),-5);
    }
    // 2740
    #[test]
    fn t_2740_ju_test(){
        assert_eq!(test_ju_money_number(2740),-15);
    }
    #[test]
    fn t_2740_goju_test(){
        assert_eq!(test_goju_money_number(2740),5);
    }
    #[test]
    fn t_2740_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2740),0);
    }
    #[test]
    fn t_2740_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2740),-5);
    }
    // 2750
    #[test]
    fn t_2750_ju_test(){
        assert_eq!(test_ju_money_number(2750),0);
    }
    #[test]
    fn t_2750_goju_test(){
        assert_eq!(test_goju_money_number(2750),0);
    }
    #[test]
    fn t_2750_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2750),0);
    }
    #[test]
    fn t_2750_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2750),-5);
    }
    // 2760
    #[test]
    fn t_2760_ju_test(){
        assert_eq!(test_ju_money_number(2760),15);
    }
    #[test]
    fn t_2760_goju_test(){
        assert_eq!(test_goju_money_number(2760),-5);
    }
    #[test]
    fn t_2760_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2760),0);
    }
    #[test]
    fn t_2760_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2760),-5);
    }
    // 2770
    #[test]
    fn t_2770_ju_test(){
        assert_eq!(test_ju_money_number(2770),5);
    }
    #[test]
    fn t_2770_goju_test(){
        assert_eq!(test_goju_money_number(2770),-5);
    }
    #[test]
    fn t_2770_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2770),0);
    }
    #[test]
    fn t_2770_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2770),-5);
    }
    // 2780
    #[test]
    fn t_2780_ju_test(){
        assert_eq!(test_ju_money_number(2780),-5);
    }
    #[test]
    fn t_2780_goju_test(){
        assert_eq!(test_goju_money_number(2780),-5);
    }
    #[test]
    fn t_2780_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2780),0);
    }
    #[test]
    fn t_2780_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2780),-5);
    }
    // 2790
    #[test]
    fn t_2790_ju_test(){
        assert_eq!(test_ju_money_number(2790),-15);
    }
    #[test]
    fn t_2790_goju_test(){
        assert_eq!(test_goju_money_number(2790),-5);
    }
    #[test]
    fn t_2790_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2790),0);
    }
    #[test]
    fn t_2790_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2790),-5);
    }
    // 2800
    #[test]
    fn t_2800_ju_test(){
        assert_eq!(test_ju_money_number(2800),0);
    }
    #[test]
    fn t_2800_goju_test(){
        assert_eq!(test_goju_money_number(2800),0);
    }
    #[test]
    fn t_2800_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2800),-5);
    }
    #[test]
    fn t_2800_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2800),-5);
    }
    // 2810
    #[test]
    fn t_2810_ju_test(){
        assert_eq!(test_ju_money_number(2810),15);
    }
    #[test]
    fn t_2810_goju_test(){
        assert_eq!(test_goju_money_number(2810),5);
    }
    #[test]
    fn t_2810_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2810),-10);
    }
    #[test]
    fn t_2810_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2810),-5);
    }
    // 2820
    #[test]
    fn t_2820_ju_test(){
        assert_eq!(test_ju_money_number(2820),5);
    }
    #[test]
    fn t_2820_goju_test(){
        assert_eq!(test_goju_money_number(2820),5);
    }
    #[test]
    fn t_2820_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2820),-10);
    }
    #[test]
    fn t_2820_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2820),-5);
    }
    // 2830
    #[test]
    fn t_2830_ju_test(){
        assert_eq!(test_ju_money_number(2830),-5);
    }
    #[test]
    fn t_2830_goju_test(){
        assert_eq!(test_goju_money_number(2830),5);
    }
    #[test]
    fn t_2830_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2830),-10);
    }
    #[test]
    fn t_2830_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2830),-5);
    }
    // 2840
    #[test]
    fn t_2840_ju_test(){
        assert_eq!(test_ju_money_number(2840),-15);
    }
    #[test]
    fn t_2840_goju_test(){
        assert_eq!(test_goju_money_number(2840),5);
    }
    #[test]
    fn t_2840_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2840),-10);
    }
    #[test]
    fn t_2840_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2840),-5);
    }
    // 2850
    #[test]
    fn t_2850_ju_test(){
        assert_eq!(test_ju_money_number(2850),0);
    }
    #[test]
    fn t_2850_goju_test(){
        assert_eq!(test_goju_money_number(2850),0);
    }
    #[test]
    fn t_2850_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2850),-10);
    }
    #[test]
    fn t_2850_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2850),-5);
    }
    // 2860
    #[test]
    fn t_2860_ju_test(){
        assert_eq!(test_ju_money_number(2860),15);
    }
    #[test]
    fn t_2860_goju_test(){
        assert_eq!(test_goju_money_number(2860),-5);
    }
    #[test]
    fn t_2860_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2860),-10);
    }
    #[test]
    fn t_2860_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2860),-5);
    }
    // 2870
    #[test]
    fn t_2870_ju_test(){
        assert_eq!(test_ju_money_number(2870),5);
    }
    #[test]
    fn t_2870_goju_test(){
        assert_eq!(test_goju_money_number(2870),-5);
    }
    #[test]
    fn t_2870_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2870),-10);
    }
    #[test]
    fn t_2870_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2870),-5);
    }
    // 2880
    #[test]
    fn t_2880_ju_test(){
        assert_eq!(test_ju_money_number(2880),-5);
    }
    #[test]
    fn t_2880_goju_test(){
        assert_eq!(test_goju_money_number(2880),-5);
    }
    #[test]
    fn t_2880_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2880),-10);
    }
    #[test]
    fn t_2880_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2880),-5);
    }
    // 2890
    #[test]
    fn t_2890_ju_test(){
        assert_eq!(test_ju_money_number(2890),-15);
    }
    #[test]
    fn t_2890_goju_test(){
        assert_eq!(test_goju_money_number(2890),-5);
    }
    #[test]
    fn t_2890_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2890),-10);
    }
    #[test]
    fn t_2890_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2890),-5);
    }
    // 2900
    #[test]
    fn t_2900_ju_test(){
        assert_eq!(test_ju_money_number(2900),0);
    }
    #[test]
    fn t_2900_goju_test(){
        assert_eq!(test_goju_money_number(2900),0);
    }
    #[test]
    fn t_2900_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2900),-15);
    }
    #[test]
    fn t_2900_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2900),-5);
    }
    // 2910
    #[test]
    fn t_2910_ju_test(){
        assert_eq!(test_ju_money_number(2910),15);
    }
    #[test]
    fn t_2910_goju_test(){
        assert_eq!(test_goju_money_number(2910),5);
    }
    #[test]
    fn t_2910_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2910),-20);
    }
    #[test]
    fn t_2910_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2910),-5);
    }
    // 2920
    #[test]
    fn t_2920_ju_test(){
        assert_eq!(test_ju_money_number(2920),5);
    }
    #[test]
    fn t_2920_goju_test(){
        assert_eq!(test_goju_money_number(2920),5);
    }
    #[test]
    fn t_2920_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2920),-20);
    }
    #[test]
    fn t_2920_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2920),-5);
    }
    // 2930
    #[test]
    fn t_2930_ju_test(){
        assert_eq!(test_ju_money_number(2930),-5);
    }
    #[test]
    fn t_2930_goju_test(){
        assert_eq!(test_goju_money_number(2930),5);
    }
    #[test]
    fn t_2930_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2930),-20);
    }
    #[test]
    fn t_2930_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2930),-5);
    }
    // 2940
    #[test]
    fn t_2940_ju_test(){
        assert_eq!(test_ju_money_number(2940),-15);
    }
    #[test]
    fn t_2940_goju_test(){
        assert_eq!(test_goju_money_number(2940),5);
    }
    #[test]
    fn t_2940_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2940),-20);
    }
    #[test]
    fn t_2940_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2940),-5);
    }
    // 2950
    #[test]
    fn t_2950_ju_test(){
        assert_eq!(test_ju_money_number(2950),0);
    }
    #[test]
    fn t_2950_goju_test(){
        assert_eq!(test_goju_money_number(2950),0);
    }
    #[test]
    fn t_2950_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2950),-20);
    }
    #[test]
    fn t_2950_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2950),-5);
    }
    // 2960
    #[test]
    fn t_2960_ju_test(){
        assert_eq!(test_ju_money_number(2960),15);
    }
    #[test]
    fn t_2960_goju_test(){
        assert_eq!(test_goju_money_number(2960),-5);
    }
    #[test]
    fn t_2960_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2960),-20);
    }
    #[test]
    fn t_2960_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2960),-5);
    }
    // 2970
    #[test]
    fn t_2970_ju_test(){
        assert_eq!(test_ju_money_number(2970),5);
    }
    #[test]
    fn t_2970_goju_test(){
        assert_eq!(test_goju_money_number(2970),-5);
    }
    #[test]
    fn t_2970_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2970),-20);
    }
    #[test]
    fn t_2970_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2970),-5);
    }
    // 2980
    #[test]
    fn t_2980_ju_test(){
        assert_eq!(test_ju_money_number(2980),-5);
    }
    #[test]
    fn t_2980_goju_test(){
        assert_eq!(test_goju_money_number(2980),-5);
    }
    #[test]
    fn t_2980_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2980),-20);
    }
    #[test]
    fn t_2980_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2980),-5);
    }
    // 2990
    #[test]
    fn t_2990_ju_test(){
        assert_eq!(test_ju_money_number(2990),-15);
    }
    #[test]
    fn t_2990_goju_test(){
        assert_eq!(test_goju_money_number(2990),-5);
    }
    #[test]
    fn t_2990_hyaku_test(){
        assert_eq!(test_hyaku_money_number(2990),-20);
    }
    #[test]
    fn t_2990_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(2990),-5);
    }
    // 3000
    #[test]
    fn t_3000_ju_test(){
        assert_eq!(test_ju_money_number(3000),0);
    }
    #[test]
    fn t_3000_goju_test(){
        assert_eq!(test_goju_money_number(3000),0);
    }
    #[test]
    fn t_3000_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3000),0);
    }
    #[test]
    fn t_3000_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3000),0);
    }
    #[test]
    fn t_3000_sen_test(){
        assert_eq!(test_sen_money_number(3000),-5);
    }
    #[test]
    fn t_3000_gosen_test(){
        assert_eq!(test_gosen_money_number(3000),-5);
    }
    // 3010
    #[test]
    fn t_3010_ju_test(){
        assert_eq!(test_ju_money_number(3010),35);
    }
    #[test]
    fn t_3010_goju_test(){
        assert_eq!(test_goju_money_number(3010),7);
    }
    #[test]
    fn t_3010_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3010),27);
    }
    #[test]
    fn t_3010_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3010),5);
    }
    #[test]
    fn t_3010_sen_test(){
        assert_eq!(test_sen_money_number(3010),-11);
    }
    #[test]
    fn t_3010_gosen_test(){
        assert_eq!(test_gosen_money_number(3010),-5);
    }
    // 3020
    #[test]
    fn t_3020_ju_test(){
        assert_eq!(test_ju_money_number(3020),25);
    }
    #[test]
    fn t_3020_goju_test(){
        assert_eq!(test_goju_money_number(3020),7);
    }
    #[test]
    fn t_3020_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3020),27);
    }
    #[test]
    fn t_3020_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3020),5);
    }
    #[test]
    fn t_3020_sen_test(){
        assert_eq!(test_sen_money_number(3020),-11);
    }
    #[test]
    fn t_3020_gosen_test(){
        assert_eq!(test_gosen_money_number(3020),-5);
    }
    // 3030
    #[test]
    fn t_3030_ju_test(){
        assert_eq!(test_ju_money_number(3030),15);
    }
    #[test]
    fn t_3030_goju_test(){
        assert_eq!(test_goju_money_number(3030),7);
    }
    #[test]
    fn t_3030_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3030),27);
    }
    #[test]
    fn t_3030_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3030),5);
    }
    #[test]
    fn t_3030_sen_test(){
        assert_eq!(test_sen_money_number(3030),-11);
    }
    #[test]
    fn t_3030_gosen_test(){
        assert_eq!(test_gosen_money_number(3030),-5);
    }
    // 3040
    #[test]
    fn t_3040_ju_test(){
        assert_eq!(test_ju_money_number(3040),5);
    }
    #[test]
    fn t_3040_goju_test(){
        assert_eq!(test_goju_money_number(3040),7);
    }
    #[test]
    fn t_3040_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3040),27);
    }
    #[test]
    fn t_3040_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3040),5);
    }
    #[test]
    fn t_3040_sen_test(){
        assert_eq!(test_sen_money_number(3040),-11);
    }
    #[test]
    fn t_3040_gosen_test(){
        assert_eq!(test_gosen_money_number(3040),-5);
    }
    // 3050
    #[test]
    fn t_3050_ju_test(){
        assert_eq!(test_ju_money_number(3050),0);
    }
    #[test]
    fn t_3050_goju_test(){
        assert_eq!(test_goju_money_number(3050),6);
    }
    #[test]
    fn t_3050_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3050),22);
    }
    #[test]
    fn t_3050_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3050),2);
    }
    #[test]
    fn t_3050_sen_test(){
        assert_eq!(test_sen_money_number(3050),-24);
    }
    #[test]
    fn t_3050_gosen_test(){
        assert_eq!(test_gosen_money_number(3050),-2);
    }
    // 3060
    #[test]
    fn t_3060_ju_test(){
        assert_eq!(test_ju_money_number(3060),30);
    }
    #[test]
    fn t_3060_goju_test(){
        assert_eq!(test_goju_money_number(3060),-2);
    }
    #[test]
    fn t_3060_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3060),22);
    }
    #[test]
    fn t_3060_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3060),2);
    }
    #[test]
    fn t_3060_sen_test(){
        assert_eq!(test_sen_money_number(3060),-24);
    }
    #[test]
    fn t_3060_gosen_test(){
        assert_eq!(test_gosen_money_number(3060),-2);
    }
    // 3070
    #[test]
    fn t_3070_ju_test(){
        assert_eq!(test_ju_money_number(3070),20);
    }
    #[test]
    fn t_3070_goju_test(){
        assert_eq!(test_goju_money_number(3070),-2);
    }
    #[test]
    fn t_3070_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3070),22);
    }
    #[test]
    fn t_3070_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3070),2);
    }
    #[test]
    fn t_3070_sen_test(){
        assert_eq!(test_sen_money_number(3070),-24);
    }
    #[test]
    fn t_3070_gosen_test(){
        assert_eq!(test_gosen_money_number(3070),-2);
    }
    // 3080
    #[test]
    fn t_3080_ju_test(){
        assert_eq!(test_ju_money_number(3080),10);
    }
    #[test]
    fn t_3080_goju_test(){
        assert_eq!(test_goju_money_number(3080),-2);
    }
    #[test]
    fn t_3080_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3080),22);
    }
    #[test]
    fn t_3080_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3080),2);
    }
    #[test]
    fn t_3080_sen_test(){
        assert_eq!(test_sen_money_number(3080),-24);
    }
    #[test]
    fn t_3080_gosen_test(){
        assert_eq!(test_gosen_money_number(3080),-2);
    }
    // 3090
    #[test]
    fn t_3090_ju_test(){
        assert_eq!(test_ju_money_number(3090),0);
    }
    #[test]
    fn t_3090_goju_test(){
        assert_eq!(test_goju_money_number(3090),-2);
    }
    #[test]
    fn t_3090_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3090),22);
    }
    #[test]
    fn t_3090_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3090),2);
    }
    #[test]
    fn t_3090_sen_test(){
        assert_eq!(test_sen_money_number(3090),-24);
    }
    #[test]
    fn t_3090_gosen_test(){
        assert_eq!(test_gosen_money_number(3090),-2);
    }
    // 3100
    #[test]
    fn t_3100_ju_test(){
        assert_eq!(test_ju_money_number(3100),0);
    }
    #[test]
    fn t_3100_goju_test(){
        assert_eq!(test_goju_money_number(3100),0);
    }
    #[test]
    fn t_3100_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3100),30);
    }
    #[test]
    fn t_3100_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3100),4);
    }
    #[test]
    fn t_3100_sen_test(){
        assert_eq!(test_sen_money_number(3100),-16);
    }
    #[test]
    fn t_3100_gosen_test(){
        assert_eq!(test_gosen_money_number(3100),-4);
    }
    // 3110
    #[test]
    fn t_3110_ju_test(){
        assert_eq!(test_ju_money_number(3110),35);
    }
    #[test]
    fn t_3110_goju_test(){
        assert_eq!(test_goju_money_number(3110),7);
    }
    #[test]
    fn t_3110_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3110),17);
    }
    #[test]
    fn t_3110_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3110),5);
    }
    #[test]
    fn t_3110_sen_test(){
        assert_eq!(test_sen_money_number(3110),-11);
    }
    #[test]
    fn t_3110_gosen_test(){
        assert_eq!(test_gosen_money_number(3110),-5);
    }
    // 3120
    #[test]
    fn t_3120_ju_test(){
        assert_eq!(test_ju_money_number(3120),25);
    }
    #[test]
    fn t_3120_goju_test(){
        assert_eq!(test_goju_money_number(3120),7);
    }
    #[test]
    fn t_3120_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3120),17);
    }
    #[test]
    fn t_3120_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3120),5);
    }
    #[test]
    fn t_3120_sen_test(){
        assert_eq!(test_sen_money_number(3120),-11);
    }
    #[test]
    fn t_3120_gosen_test(){
        assert_eq!(test_gosen_money_number(3120),-5);
    }
    // 3130
    #[test]
    fn t_3130_ju_test(){
        assert_eq!(test_ju_money_number(3130),15);
    }
    #[test]
    fn t_3130_goju_test(){
        assert_eq!(test_goju_money_number(3130),7);
    }
    #[test]
    fn t_3130_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3130),17);
    }
    #[test]
    fn t_3130_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3130),5);
    }
    #[test]
    fn t_3130_sen_test(){
        assert_eq!(test_sen_money_number(3130),-11);
    }
    #[test]
    fn t_3130_gosen_test(){
        assert_eq!(test_gosen_money_number(3130),-5);
    }
    // 3140
    #[test]
    fn t_3140_ju_test(){
        assert_eq!(test_ju_money_number(3140),5);
    }
    #[test]
    fn t_3140_goju_test(){
        assert_eq!(test_goju_money_number(3140),7);
    }
    #[test]
    fn t_3140_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3140),17);
    }
    #[test]
    fn t_3140_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3140),5);
    }
    #[test]
    fn t_3140_sen_test(){
        assert_eq!(test_sen_money_number(3140),-11);
    }
    #[test]
    fn t_3140_gosen_test(){
        assert_eq!(test_gosen_money_number(3140),-5);
    }
    // 3150
    #[test]
    fn t_3150_ju_test(){
        assert_eq!(test_ju_money_number(3150),0);
    }
    #[test]
    fn t_3150_goju_test(){
        assert_eq!(test_goju_money_number(3150),6);
    }
    #[test]
    fn t_3150_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3150),12);
    }
    #[test]
    fn t_3150_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3150),2);
    }
    #[test]
    fn t_3150_sen_test(){
        assert_eq!(test_sen_money_number(3150),-24);
    }
    #[test]
    fn t_3150_gosen_test(){
        assert_eq!(test_gosen_money_number(3150),-2);
    }
    // 3160
    #[test]
    fn t_3160_ju_test(){
        assert_eq!(test_ju_money_number(3160),30);
    }
    #[test]
    fn t_3160_goju_test(){
        assert_eq!(test_goju_money_number(3160),-2);
    }
    #[test]
    fn t_3160_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3160),12);
    }
    #[test]
    fn t_3160_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3160),2);
    }
    #[test]
    fn t_3160_sen_test(){
        assert_eq!(test_sen_money_number(3160),-24);
    }
    #[test]
    fn t_3160_gosen_test(){
        assert_eq!(test_gosen_money_number(3160),-2);
    }
    // 3170
    #[test]
    fn t_3170_ju_test(){
        assert_eq!(test_ju_money_number(3170),20);
    }
    #[test]
    fn t_3170_goju_test(){
        assert_eq!(test_goju_money_number(3170),-2);
    }
    #[test]
    fn t_3170_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3170),12);
    }
    #[test]
    fn t_3170_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3170),2);
    }
    #[test]
    fn t_3170_sen_test(){
        assert_eq!(test_sen_money_number(3170),-24);
    }
    #[test]
    fn t_3170_gosen_test(){
        assert_eq!(test_gosen_money_number(3170),-2);
    }
    // 3180
    #[test]
    fn t_3180_ju_test(){
        assert_eq!(test_ju_money_number(3180),10);
    }
    #[test]
    fn t_3180_goju_test(){
        assert_eq!(test_goju_money_number(3180),-2);
    }
    #[test]
    fn t_3180_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3180),12);
    }
    #[test]
    fn t_3180_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3180),2);
    }
    #[test]
    fn t_3180_sen_test(){
        assert_eq!(test_sen_money_number(3180),-24);
    }
    #[test]
    fn t_3180_gosen_test(){
        assert_eq!(test_gosen_money_number(3180),-2);
    }
    // 3190
    #[test]
    fn t_3190_ju_test(){
        assert_eq!(test_ju_money_number(3190),0);
    }
    #[test]
    fn t_3190_goju_test(){
        assert_eq!(test_goju_money_number(3190),-2);
    }
    #[test]
    fn t_3190_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3190),12);
    }
    #[test]
    fn t_3190_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3190),2);
    }
    #[test]
    fn t_3190_sen_test(){
        assert_eq!(test_sen_money_number(3190),-24);
    }
    #[test]
    fn t_3190_gosen_test(){
        assert_eq!(test_gosen_money_number(3190),-2);
    }
    // 3200
    #[test]
    fn t_3200_ju_test(){
        assert_eq!(test_ju_money_number(3200),0);
    }
    #[test]
    fn t_3200_goju_test(){
        assert_eq!(test_goju_money_number(3200),0);
    }
    #[test]
    fn t_3200_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3200),20);
    }
    #[test]
    fn t_3200_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3200),4);
    }
    #[test]
    fn t_3200_sen_test(){
        assert_eq!(test_sen_money_number(3200),-16);
    }
    #[test]
    fn t_3200_gosen_test(){
        assert_eq!(test_gosen_money_number(3200),-4);
    }
    // 3210
    #[test]
    fn t_3210_ju_test(){
        assert_eq!(test_ju_money_number(3210),35);
    }
    #[test]
    fn t_3210_goju_test(){
        assert_eq!(test_goju_money_number(3210),7);
    }
    #[test]
    fn t_3210_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3210),7);
    }
    #[test]
    fn t_3210_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3210),5);
    }
    #[test]
    fn t_3210_sen_test(){
        assert_eq!(test_sen_money_number(3210),-11);
    }
    #[test]
    fn t_3210_gosen_test(){
        assert_eq!(test_gosen_money_number(3210),-5);
    }
    // 3220
    #[test]
    fn t_3220_ju_test(){
        assert_eq!(test_ju_money_number(3220),25);
    }
    #[test]
    fn t_3220_goju_test(){
        assert_eq!(test_goju_money_number(3220),7);
    }
    #[test]
    fn t_3220_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3220),7);
    }
    #[test]
    fn t_3220_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3220),5);
    }
    #[test]
    fn t_3220_sen_test(){
        assert_eq!(test_sen_money_number(3220),-11);
    }
    #[test]
    fn t_3220_gosen_test(){
        assert_eq!(test_gosen_money_number(3220),-5);
    }
    // 3230
    #[test]
    fn t_3230_ju_test(){
        assert_eq!(test_ju_money_number(3230),15);
    }
    #[test]
    fn t_3230_goju_test(){
        assert_eq!(test_goju_money_number(3230),7);
    }
    #[test]
    fn t_3230_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3230),7);
    }
    #[test]
    fn t_3230_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3230),5);
    }
    #[test]
    fn t_3230_sen_test(){
        assert_eq!(test_sen_money_number(3230),-11);
    }
    #[test]
    fn t_3230_gosen_test(){
        assert_eq!(test_gosen_money_number(3230),-5);
    }
    // 3240
    #[test]
    fn t_3240_ju_test(){
        assert_eq!(test_ju_money_number(3240),5);
    }
    #[test]
    fn t_3240_goju_test(){
        assert_eq!(test_goju_money_number(3240),7);
    }
    #[test]
    fn t_3240_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3240),7);
    }
    #[test]
    fn t_3240_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3240),5);
    }
    #[test]
    fn t_3240_sen_test(){
        assert_eq!(test_sen_money_number(3240),-11);
    }
    #[test]
    fn t_3240_gosen_test(){
        assert_eq!(test_gosen_money_number(3240),-5);
    }
    // 3250
    #[test]
    fn t_3250_ju_test(){
        assert_eq!(test_ju_money_number(3250),0);
    }
    #[test]
    fn t_3250_goju_test(){
        assert_eq!(test_goju_money_number(3250),6);
    }
    #[test]
    fn t_3250_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3250),2);
    }
    #[test]
    fn t_3250_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3250),2);
    }
    #[test]
    fn t_3250_sen_test(){
        assert_eq!(test_sen_money_number(3250),-24);
    }
    #[test]
    fn t_3250_gosen_test(){
        assert_eq!(test_gosen_money_number(3250),-2);
    }
    // 3260
    #[test]
    fn t_3260_ju_test(){
        assert_eq!(test_ju_money_number(3260),30);
    }
    #[test]
    fn t_3260_goju_test(){
        assert_eq!(test_goju_money_number(3260),-2);
    }
    #[test]
    fn t_3260_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3260),2);
    }
    #[test]
    fn t_3260_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3260),2);
    }
    #[test]
    fn t_3260_sen_test(){
        assert_eq!(test_sen_money_number(3260),-24);
    }
    #[test]
    fn t_3260_gosen_test(){
        assert_eq!(test_gosen_money_number(3260),-2);
    }
    // 3270
    #[test]
    fn t_3270_ju_test(){
        assert_eq!(test_ju_money_number(3270),20);
    }
    #[test]
    fn t_3270_goju_test(){
        assert_eq!(test_goju_money_number(3270),-2);
    }
    #[test]
    fn t_3270_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3270),2);
    }
    #[test]
    fn t_3270_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3270),2);
    }
    #[test]
    fn t_3270_sen_test(){
        assert_eq!(test_sen_money_number(3270),-24);
    }
    #[test]
    fn t_3270_gosen_test(){
        assert_eq!(test_gosen_money_number(3270),-2);
    }
    // 3280
    #[test]
    fn t_3280_ju_test(){
        assert_eq!(test_ju_money_number(3280),10);
    }
    #[test]
    fn t_3280_goju_test(){
        assert_eq!(test_goju_money_number(3280),-2);
    }
    #[test]
    fn t_3280_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3280),2);
    }
    #[test]
    fn t_3280_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3280),2);
    }
    #[test]
    fn t_3280_sen_test(){
        assert_eq!(test_sen_money_number(3280),-24);
    }
    #[test]
    fn t_3280_gosen_test(){
        assert_eq!(test_gosen_money_number(3280),-2);
    }
    // 3290
    #[test]
    fn t_3290_ju_test(){
        assert_eq!(test_ju_money_number(3290),0);
    }
    #[test]
    fn t_3290_goju_test(){
        assert_eq!(test_goju_money_number(3290),-2);
    }
    #[test]
    fn t_3290_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3290),2);
    }
    #[test]
    fn t_3290_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3290),2);
    }
    #[test]
    fn t_3290_sen_test(){
        assert_eq!(test_sen_money_number(3290),-24);
    }
    #[test]
    fn t_3290_gosen_test(){
        assert_eq!(test_gosen_money_number(3290),-2);
    }
    // 3300
    #[test]
    fn t_3300_ju_test(){
        assert_eq!(test_ju_money_number(3300),0);
    }
    #[test]
    fn t_3300_goju_test(){
        assert_eq!(test_goju_money_number(3300),0);
    }
    #[test]
    fn t_3300_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3300),10);
    }
    #[test]
    fn t_3300_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3300),4);
    }
    #[test]
    fn t_3300_sen_test(){
        assert_eq!(test_sen_money_number(3300),-16);
    }
    #[test]
    fn t_3300_gosen_test(){
        assert_eq!(test_gosen_money_number(3300),-4);
    }
    // 3310
    #[test]
    fn t_3310_ju_test(){
        assert_eq!(test_ju_money_number(3310),35);
    }
    #[test]
    fn t_3310_goju_test(){
        assert_eq!(test_goju_money_number(3310),7);
    }
    #[test]
    fn t_3310_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3310),-3);
    }
    #[test]
    fn t_3310_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3310),5);
    }
    #[test]
    fn t_3310_sen_test(){
        assert_eq!(test_sen_money_number(3310),-11);
    }
    #[test]
    fn t_3310_gosen_test(){
        assert_eq!(test_gosen_money_number(3310),-5);
    }
    // 3320
    #[test]
    fn t_3320_ju_test(){
        assert_eq!(test_ju_money_number(3220),25);
    }
    #[test]
    fn t_3320_goju_test(){
        assert_eq!(test_goju_money_number(3320),7);
    }
    #[test]
    fn t_3320_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3320),-3);
    }
    #[test]
    fn t_3320_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3320),5);
    }
    #[test]
    fn t_3320_sen_test(){
        assert_eq!(test_sen_money_number(3320),-11);
    }
    #[test]
    fn t_3320_gosen_test(){
        assert_eq!(test_gosen_money_number(3320),-5);
    }
    // 3330
    #[test]
    fn t_3330_ju_test(){
        assert_eq!(test_ju_money_number(3330),15);
    }
    #[test]
    fn t_3330_goju_test(){
        assert_eq!(test_goju_money_number(3330),7);
    }
    #[test]
    fn t_3330_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3330),-3);
    }
    #[test]
    fn t_3330_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3330),5);
    }
    #[test]
    fn t_3330_sen_test(){
        assert_eq!(test_sen_money_number(3330),-11);
    }
    #[test]
    fn t_3330_gosen_test(){
        assert_eq!(test_gosen_money_number(3330),-5);
    }
    // 3340
    #[test]
    fn t_3340_ju_test(){
        assert_eq!(test_ju_money_number(3340),5);
    }
    #[test]
    fn t_3340_goju_test(){
        assert_eq!(test_goju_money_number(3340),7);
    }
    #[test]
    fn t_3340_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3340),-3);
    }
    #[test]
    fn t_3340_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3340),5);
    }
    #[test]
    fn t_3340_sen_test(){
        assert_eq!(test_sen_money_number(3340),-11);
    }
    #[test]
    fn t_3340_gosen_test(){
        assert_eq!(test_gosen_money_number(3340),-5);
    }
    // 3350
    #[test]
    fn t_3350_ju_test(){
        assert_eq!(test_ju_money_number(3350),0);
    }
    #[test]
    fn t_3350_goju_test(){
        assert_eq!(test_goju_money_number(3350),6);
    }
    #[test]
    fn t_3350_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3350),-8);
    }
    #[test]
    fn t_3350_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3350),2);
    }
    #[test]
    fn t_3350_sen_test(){
        assert_eq!(test_sen_money_number(3350),-24);
    }
    #[test]
    fn t_3350_gosen_test(){
        assert_eq!(test_gosen_money_number(3350),-2);
    }
    // 3360
    #[test]
    fn t_3360_ju_test(){
        assert_eq!(test_ju_money_number(3360),30);
    }
    #[test]
    fn t_3360_goju_test(){
        assert_eq!(test_goju_money_number(3360),-2);
    }
    #[test]
    fn t_3360_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3360),-8);
    }
    #[test]
    fn t_3360_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3360),2);
    }
    #[test]
    fn t_3360_sen_test(){
        assert_eq!(test_sen_money_number(3360),-24);
    }
    #[test]
    fn t_3360_gosen_test(){
        assert_eq!(test_gosen_money_number(3360),-2);
    }
    // 3370
    #[test]
    fn t_3370_ju_test(){
        assert_eq!(test_ju_money_number(3370),20);
    }
    #[test]
    fn t_3370_goju_test(){
        assert_eq!(test_goju_money_number(3370),-2);
    }
    #[test]
    fn t_3370_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3370),-8);
    }
    #[test]
    fn t_3370_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3370),2);
    }
    #[test]
    fn t_3370_sen_test(){
        assert_eq!(test_sen_money_number(3370),-24);
    }
    #[test]
    fn t_3370_gosen_test(){
        assert_eq!(test_gosen_money_number(3370),-2);
    }
    // 3380
    #[test]
    fn t_3380_ju_test(){
        assert_eq!(test_ju_money_number(3380),10);
    }
    #[test]
    fn t_3380_goju_test(){
        assert_eq!(test_goju_money_number(3380),-2);
    }
    #[test]
    fn t_3380_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3380),-8);
    }
    #[test]
    fn t_3380_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3380),2);
    }
    #[test]
    fn t_3380_sen_test(){
        assert_eq!(test_sen_money_number(3380),-24);
    }
    #[test]
    fn t_3380_gosen_test(){
        assert_eq!(test_gosen_money_number(3380),-2);
    }
    // 3390
    #[test]
    fn t_3390_ju_test(){
        assert_eq!(test_ju_money_number(3390),0);
    }
    #[test]
    fn t_3390_goju_test(){
        assert_eq!(test_goju_money_number(3390),-2);
    }
    #[test]
    fn t_3390_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3390),-8);
    }
    #[test]
    fn t_3390_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3390),2);
    }
    #[test]
    fn t_3390_sen_test(){
        assert_eq!(test_sen_money_number(3390),-24);
    }
    #[test]
    fn t_3390_gosen_test(){
        assert_eq!(test_gosen_money_number(3390),-2);
    }
    // 3400
    #[test]
    fn t_3400_ju_test(){
        assert_eq!(test_ju_money_number(3400),0);
    }
    #[test]
    fn t_3400_goju_test(){
        assert_eq!(test_goju_money_number(3400),0);
    }
    #[test]
    fn t_3400_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3400),0);
    }
    #[test]
    fn t_3400_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3400),4);
    }
    #[test]
    fn t_3400_sen_test(){
        assert_eq!(test_sen_money_number(3400),-16);
    }
    #[test]
    fn t_3400_gosen_test(){
        assert_eq!(test_gosen_money_number(3400),-4);
    }
    // 3410
    #[test]
    fn t_3410_ju_test(){
        assert_eq!(test_ju_money_number(3410),30);
    }
    #[test]
    fn t_3410_goju_test(){
        assert_eq!(test_goju_money_number(3410),4);
    }
    #[test]
    fn t_3410_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3410),-16);
    }
    #[test]
    fn t_3410_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3410),2);
    }
    #[test]
    fn t_3410_sen_test(){
        assert_eq!(test_sen_money_number(3410),-24);
    }
    #[test]
    fn t_3410_gosen_test(){
        assert_eq!(test_gosen_money_number(3410),-2);
    }
    // 3420
    #[test]
    fn t_3420_ju_test(){
        assert_eq!(test_ju_money_number(3420),20);
    }
    #[test]
    fn t_3420_goju_test(){
        assert_eq!(test_goju_money_number(3420),4);
    }
    #[test]
    fn t_3420_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3420),-16);
    }
    #[test]
    fn t_3420_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3420),2);
    }
    #[test]
    fn t_3420_sen_test(){
        assert_eq!(test_sen_money_number(3420),-24);
    }
    #[test]
    fn t_3420_gosen_test(){
        assert_eq!(test_gosen_money_number(3420),-2);
    }
    // 3440
    #[test]
    fn t_3440_ju_test(){
        assert_eq!(test_ju_money_number(3440),0);
    }
    #[test]
    fn t_3440_goju_test(){
        assert_eq!(test_goju_money_number(3440),4);
    }
    #[test]
    fn t_3440_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3440),-16);
    }
    #[test]
    fn t_3440_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3440),2);
    }
    #[test]
    fn t_3440_sen_test(){
        assert_eq!(test_sen_money_number(3440),-24);
    }
    #[test]
    fn t_3440_gosen_test(){
        assert_eq!(test_gosen_money_number(3440),-2);
    }
    // 3450
    #[test]
    fn t_3450_ju_test(){
        assert_eq!(test_ju_money_number(3450),0);
    }
    #[test]
    fn t_3450_goju_test(){
        assert_eq!(test_goju_money_number(3450),6);
    }
    #[test]
    fn t_3450_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3450),-8);
    }
    #[test]
    fn t_3450_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3450),4);
    }
    #[test]
    fn t_3450_sen_test(){
        assert_eq!(test_sen_money_number(3450),-16);
    }
    #[test]
    fn t_3450_gosen_test(){
        assert_eq!(test_gosen_money_number(3450),-4);
    }
    // 3460
    #[test]
    fn t_3460_ju_test(){
        assert_eq!(test_ju_money_number(3460),30);
    }
    #[test]
    fn t_3460_goju_test(){
        assert_eq!(test_goju_money_number(3460),-2);
    }
    #[test]
    fn t_3460_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3460),-8);
    }
    #[test]
    fn t_3460_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3460),4);
    }
    #[test]
    fn t_3460_sen_test(){
        assert_eq!(test_sen_money_number(3460),-16);
    }
    #[test]
    fn t_3460_gosen_test(){
        assert_eq!(test_gosen_money_number(3460),-4);
    }
    // 3470
    #[test]
    fn t_3470_ju_test(){
        assert_eq!(test_ju_money_number(3470),20);
    }
    #[test]
    fn t_3470_goju_test(){
        assert_eq!(test_goju_money_number(3470),-2);
    }
    #[test]
    fn t_3470_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3470),-8);
    }
    #[test]
    fn t_3470_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3470),4);
    }
    #[test]
    fn t_3470_sen_test(){
        assert_eq!(test_sen_money_number(3470),-16);
    }
    #[test]
    fn t_3470_gosen_test(){
        assert_eq!(test_gosen_money_number(3470),-4);
    }
    // 3480
    #[test]
    fn t_3480_ju_test(){
        assert_eq!(test_ju_money_number(3480),10);
    }
    #[test]
    fn t_3480_goju_test(){
        assert_eq!(test_goju_money_number(3480),-2);
    }
    #[test]
    fn t_3480_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3480),-8);
    }
    #[test]
    fn t_3480_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3480),4);
    }
    #[test]
    fn t_3480_sen_test(){
        assert_eq!(test_sen_money_number(3480),-16);
    }
    #[test]
    fn t_3480_gosen_test(){
        assert_eq!(test_gosen_money_number(3480),-4);
    }
    // 3490
    #[test]
    fn t_3490_ju_test(){
        assert_eq!(test_ju_money_number(3490),0);
    }
    #[test]
    fn t_3490_goju_test(){
        assert_eq!(test_goju_money_number(3490),-2);
    }
    #[test]
    fn t_3490_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3490),-8);
    }
    #[test]
    fn t_3490_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3490),4);
    }
    #[test]
    fn t_3490_sen_test(){
        assert_eq!(test_sen_money_number(3490),-16);
    }
    #[test]
    fn t_3490_gosen_test(){
        assert_eq!(test_gosen_money_number(3490),-4);
    }
    // 3500
    #[test]
    fn t_3500_ju_test(){
        assert_eq!(test_ju_money_number(3500),0);
    }
    #[test]
    fn t_3500_goju_test(){
        assert_eq!(test_goju_money_number(3500),0);
    }
    #[test]
    fn t_3500_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3500),0);
    }
    #[test]
    fn t_3500_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3500),4);
    }
    #[test]
    fn t_3500_sen_test(){
        assert_eq!(test_sen_money_number(3500),-17);
    }
    #[test]
    fn t_3500_gosen_test(){
        assert_eq!(test_gosen_money_number(3500),-4);
    }
    // 3510
    #[test]
    fn t_3510_ju_test(){
        assert_eq!(test_ju_money_number(3510),30);
    }
    #[test]
    fn t_3510_goju_test(){
        assert_eq!(test_goju_money_number(3510),4);
    }
    #[test]
    fn t_3510_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3510),14);
    }
    #[test]
    fn t_3510_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3510),-6);
    }
    #[test]
    fn t_3510_sen_test(){
        assert_eq!(test_sen_money_number(3510),-24);
    }
    #[test]
    fn t_3510_gosen_test(){
        assert_eq!(test_gosen_money_number(3510),-2);
    }
    // 3520
    #[test]
    fn t_3520_ju_test(){
        assert_eq!(test_ju_money_number(3520),20);
    }
    #[test]
    fn t_3520_goju_test(){
        assert_eq!(test_goju_money_number(3520),4);
    }
    #[test]
    fn t_3520_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3520),14);
    }
    #[test]
    fn t_3520_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3520),-6);
    }
    #[test]
    fn t_3520_sen_test(){
        assert_eq!(test_sen_money_number(3520),-24);
    }
    #[test]
    fn t_3520_gosen_test(){
        assert_eq!(test_gosen_money_number(3520),-2);
    }
    // 3530
    #[test]
    fn t_3530_ju_test(){
        assert_eq!(test_ju_money_number(3530),10);
    }
    #[test]
    fn t_3530_goju_test(){
        assert_eq!(test_goju_money_number(3530),4);
    }
    #[test]
    fn t_3530_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3530),14);
    }
    #[test]
    fn t_3530_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3530),-6);
    }
    #[test]
    fn t_3530_sen_test(){
        assert_eq!(test_sen_money_number(3530),-24);
    }
    #[test]
    fn t_3530_gosen_test(){
        assert_eq!(test_gosen_money_number(3530),-2);
    }
    // 3540
    #[test]
    fn t_3540_ju_test(){
        assert_eq!(test_ju_money_number(3540),0);
    }
    #[test]
    fn t_3540_goju_test(){
        assert_eq!(test_goju_money_number(3540),4);
    }
    #[test]
    fn t_3540_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3540),14);
    }
    #[test]
    fn t_3540_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3540),-6);
    }
    #[test]
    fn t_3540_sen_test(){
        assert_eq!(test_sen_money_number(3540),-24);
    }
    #[test]
    fn t_3540_gosen_test(){
        assert_eq!(test_gosen_money_number(3540),-2);
    }
    // 3550
    #[test]
    fn t_3550_ju_test(){
        assert_eq!(test_ju_money_number(3550),0);
    }
    #[test]
    fn t_3550_goju_test(){
        assert_eq!(test_goju_money_number(3550),6);
    }
    #[test]
    fn t_3550_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3550),22);
    }
    #[test]
    fn t_3550_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3550),-4);
    }
    #[test]
    fn t_3550_sen_test(){
        assert_eq!(test_sen_money_number(3550),-16);
    }
    #[test]
    fn t_3550_gosen_test(){
        assert_eq!(test_gosen_money_number(3550),-4);
    }
    // 3560
    #[test]
    fn t_3560_ju_test(){
        assert_eq!(test_ju_money_number(3560),30);
    }
    #[test]
    fn t_3560_goju_test(){
        assert_eq!(test_goju_money_number(3560),-2);
    }
    #[test]
    fn t_3560_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3560),22);
    }
    #[test]
    fn t_3560_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3560),-4);
    }
    #[test]
    fn t_3560_sen_test(){
        assert_eq!(test_sen_money_number(3560),-16);
    }
    #[test]
    fn t_3560_gosen_test(){
        assert_eq!(test_gosen_money_number(3560),-4);
    }
    // 3570
    #[test]
    fn t_3570_ju_test(){
        assert_eq!(test_ju_money_number(3570),20);
    }
    #[test]
    fn t_3570_goju_test(){
        assert_eq!(test_goju_money_number(3570),-2);
    }
    #[test]
    fn t_3570_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3570),22);
    }
    #[test]
    fn t_3570_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3570),-4);
    }
    #[test]
    fn t_3570_sen_test(){
        assert_eq!(test_sen_money_number(3570),-16);
    }
    #[test]
    fn t_3570_gosen_test(){
        assert_eq!(test_gosen_money_number(3570),-4);
    }
    // 3580
    #[test]
    fn t_3580_ju_test(){
        assert_eq!(test_ju_money_number(3580),10);
    }
    #[test]
    fn t_3580_goju_test(){
        assert_eq!(test_goju_money_number(3580),-2);
    }
    #[test]
    fn t_3580_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3580),22);
    }
    #[test]
    fn t_3580_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3580),-4);
    }
    #[test]
    fn t_3580_sen_test(){
        assert_eq!(test_sen_money_number(3580),-16);
    }
    #[test]
    fn t_3580_gosen_test(){
        assert_eq!(test_gosen_money_number(3580),-4);
    }
    // 3590
    #[test]
    fn t_3590_ju_test(){
        assert_eq!(test_ju_money_number(3590),0);
    }
    #[test]
    fn t_3590_goju_test(){
        assert_eq!(test_goju_money_number(3590),-2);
    }
    #[test]
    fn t_3590_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3590),22);
    }
    #[test]
    fn t_3590_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3590),-4);
    }
    #[test]
    fn t_3590_sen_test(){
        assert_eq!(test_sen_money_number(3590),-16);
    }
    #[test]
    fn t_3590_gosen_test(){
        assert_eq!(test_gosen_money_number(3590),-4);
    }
    // 3600
    #[test]
    fn t_3600_ju_test(){
        assert_eq!(test_ju_money_number(3600),0);
    }
    #[test]
    fn t_3600_goju_test(){
        assert_eq!(test_goju_money_number(3600),0);
    }
    #[test]
    fn t_3600_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3600),25);
    }
    #[test]
    fn t_3600_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3600),-3);
    }
    #[test]
    fn t_3600_sen_test(){
        assert_eq!(test_sen_money_number(3600),-17);
    }
    #[test]
    fn t_3600_gosen_test(){
        assert_eq!(test_gosen_money_number(3600),-4);
    }
    // 3610
    #[test]
    fn t_3610_ju_test(){
        assert_eq!(test_ju_money_number(3610),30);
    }
    #[test]
    fn t_3610_goju_test(){
        assert_eq!(test_goju_money_number(3610),4);
    }
    #[test]
    fn t_3610_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3610),4);
    }
    #[test]
    fn t_3610_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3610),-6);
    }
    #[test]
    fn t_3610_sen_test(){
        assert_eq!(test_sen_money_number(3610),-24);
    }
    #[test]
    fn t_3610_gosen_test(){
        assert_eq!(test_gosen_money_number(3610),-2);
    }
    // 3620
    #[test]
    fn t_3620_ju_test(){
        assert_eq!(test_ju_money_number(3620),20);
    }
    #[test]
    fn t_3620_goju_test(){
        assert_eq!(test_goju_money_number(3620),4);
    }
    #[test]
    fn t_3620_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3620),4);
    }
    #[test]
    fn t_3620_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3620),-6);
    }
    #[test]
    fn t_3620_sen_test(){
        assert_eq!(test_sen_money_number(3620),-24);
    }
    #[test]
    fn t_3620_gosen_test(){
        assert_eq!(test_gosen_money_number(3620),-2);
    }
    // 3630
    #[test]
    fn t_3630_ju_test(){
        assert_eq!(test_ju_money_number(3630),10);
    }
    #[test]
    fn t_3630_goju_test(){
        assert_eq!(test_goju_money_number(3630),4);
    }
    #[test]
    fn t_3630_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3630),4);
    }
    #[test]
    fn t_3630_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3630),-6);
    }
    #[test]
    fn t_3630_sen_test(){
        assert_eq!(test_sen_money_number(3630),-24);
    }
    #[test]
    fn t_3630_gosen_test(){
        assert_eq!(test_gosen_money_number(3630),-2);
    }
    // 3640
    #[test]
    fn t_3640_ju_test(){
        assert_eq!(test_ju_money_number(3640),0);
    }
    #[test]
    fn t_3640_goju_test(){
        assert_eq!(test_goju_money_number(3640),4);
    }
    #[test]
    fn t_3640_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3640),4);
    }
    #[test]
    fn t_3640_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3640),-6);
    }
    #[test]
    fn t_3640_sen_test(){
        assert_eq!(test_sen_money_number(3640),-24);
    }
    #[test]
    fn t_3640_gosen_test(){
        assert_eq!(test_gosen_money_number(3640),-2);
    }
    // 3650
    #[test]
    fn t_3650_ju_test(){
        assert_eq!(test_ju_money_number(3650),0);
    }
    #[test]
    fn t_3650_goju_test(){
        assert_eq!(test_goju_money_number(3650),6);
    }
    #[test]
    fn t_3650_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3650),12);
    }
    #[test]
    fn t_3650_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3650),-4);
    }
    #[test]
    fn t_3650_sen_test(){
        assert_eq!(test_sen_money_number(3650),-16);
    }
    #[test]
    fn t_3650_gosen_test(){
        assert_eq!(test_gosen_money_number(3650),-4);
    }
    // 3660
    #[test]
    fn t_3660_ju_test(){
        assert_eq!(test_ju_money_number(3660),30);
    }
    #[test]
    fn t_3660_goju_test(){
        assert_eq!(test_goju_money_number(3660),-2);
    }
    #[test]
    fn t_3660_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3660),12);
    }
    #[test]
    fn t_3660_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3660),-4);
    }
    #[test]
    fn t_3660_sen_test(){
        assert_eq!(test_sen_money_number(3660),-16);
    }
    #[test]
    fn t_3660_gosen_test(){
        assert_eq!(test_gosen_money_number(3660),-4);
    }
    // 3670
    #[test]
    fn t_3670_ju_test(){
        assert_eq!(test_ju_money_number(3670),20);
    }
    #[test]
    fn t_3670_goju_test(){
        assert_eq!(test_goju_money_number(3670),-2);
    }
    #[test]
    fn t_3670_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3670),12);
    }
    #[test]
    fn t_3670_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3670),-4);
    }
    #[test]
    fn t_3670_sen_test(){
        assert_eq!(test_sen_money_number(3670),-16);
    }
    #[test]
    fn t_3670_gosen_test(){
        assert_eq!(test_gosen_money_number(3670),-4);
    }
    // 3680
    #[test]
    fn t_3680_ju_test(){
        assert_eq!(test_ju_money_number(3680),10);
    }
    #[test]
    fn t_3680_goju_test(){
        assert_eq!(test_goju_money_number(3680),-2);
    }
    #[test]
    fn t_3680_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3680),12);
    }
    #[test]
    fn t_3680_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3680),-4);
    }
    #[test]
    fn t_3680_sen_test(){
        assert_eq!(test_sen_money_number(3680),-16);
    }
    #[test]
    fn t_3680_gosen_test(){
        assert_eq!(test_gosen_money_number(3680),-4);
    }
    // 3690
    #[test]
    fn t_3690_ju_test(){
        assert_eq!(test_ju_money_number(3690),0);
    }
    #[test]
    fn t_3690_goju_test(){
        assert_eq!(test_goju_money_number(3690),-2);
    }
    #[test]
    fn t_3690_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3690),12);
    }
    #[test]
    fn t_3690_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3690),-4);
    }
    #[test]
    fn t_3690_sen_test(){
        assert_eq!(test_sen_money_number(3690),-16);
    }
    #[test]
    fn t_3690_gosen_test(){
        assert_eq!(test_gosen_money_number(3690),-4);
    }
    // 3700
    #[test]
    fn t_3700_ju_test(){
        assert_eq!(test_ju_money_number(3700),0);
    }
    #[test]
    fn t_3700_goju_test(){
        assert_eq!(test_goju_money_number(3700),0);
    }
    #[test]
    fn t_3700_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3700),15);
    }
    #[test]
    fn t_3700_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3700),-3);
    }
    #[test]
    fn t_3700_sen_test(){
        assert_eq!(test_sen_money_number(3700),-17);
    }
    #[test]
    fn t_3700_gosen_test(){
        assert_eq!(test_gosen_money_number(3700),-4);
    }
    // 3710
    #[test]
    fn t_3710_ju_test(){
        assert_eq!(test_ju_money_number(3710),30);
    }
    #[test]
    fn t_3710_goju_test(){
        assert_eq!(test_goju_money_number(3710),4);
    }
    #[test]
    fn t_3710_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3710),-6);
    }
    #[test]
    fn t_3710_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3710),-6);
    }
    #[test]
    fn t_3710_sen_test(){
        assert_eq!(test_sen_money_number(3710),-24);
    }
    #[test]
    fn t_3710_gosen_test(){
        assert_eq!(test_gosen_money_number(3710),-2);
    }
    // 3720
    #[test]
    fn t_3720_ju_test(){
        assert_eq!(test_ju_money_number(3720),20);
    }
    #[test]
    fn t_3720_goju_test(){
        assert_eq!(test_goju_money_number(3720),4);
    }
    #[test]
    fn t_3720_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3720),-6);
    }
    #[test]
    fn t_3720_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3720),-6);
    }
    #[test]
    fn t_3720_sen_test(){
        assert_eq!(test_sen_money_number(3720),-24);
    }
    #[test]
    fn t_3720_gosen_test(){
        assert_eq!(test_gosen_money_number(3720),-2);
    }
    // 3730
    #[test]
    fn t_3730_ju_test(){
        assert_eq!(test_ju_money_number(3730),10);
    }
    #[test]
    fn t_3730_goju_test(){
        assert_eq!(test_goju_money_number(3730),4);
    }
    #[test]
    fn t_3730_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3730),-6);
    }
    #[test]
    fn t_3730_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3730),-6);
    }
    #[test]
    fn t_3730_sen_test(){
        assert_eq!(test_sen_money_number(3730),-24);
    }
    #[test]
    fn t_3730_gosen_test(){
        assert_eq!(test_gosen_money_number(3730),-2);
    }
    // 3740
    #[test]
    fn t_3740_ju_test(){
        assert_eq!(test_ju_money_number(3740),0);
    }
    #[test]
    fn t_3740_goju_test(){
        assert_eq!(test_goju_money_number(3740),4);
    }
    #[test]
    fn t_3740_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3740),-6);
    }
    #[test]
    fn t_3740_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3740),-6);
    }
    #[test]
    fn t_3740_sen_test(){
        assert_eq!(test_sen_money_number(3740),-24);
    }
    #[test]
    fn t_3740_gosen_test(){
        assert_eq!(test_gosen_money_number(3740),-2);
    }
    // 3750
    #[test]
    fn t_3750_ju_test(){
        assert_eq!(test_ju_money_number(3750),0);
    }
    #[test]
    fn t_3750_goju_test(){
        assert_eq!(test_goju_money_number(3750),6);
    }
    #[test]
    fn t_3750_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3750),2);
    }
    #[test]
    fn t_3750_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3750),-4);
    }
    #[test]
    fn t_3750_sen_test(){
        assert_eq!(test_sen_money_number(3750),-16);
    }
    #[test]
    fn t_3750_gosen_test(){
        assert_eq!(test_gosen_money_number(3750),-4);
    }
    // 3760
    #[test]
    fn t_3760_ju_test(){
        assert_eq!(test_ju_money_number(3760),30);
    }
    #[test]
    fn t_3760_goju_test(){
        assert_eq!(test_goju_money_number(3760),-2);
    }
    #[test]
    fn t_3760_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3760),2);
    }
    #[test]
    fn t_3760_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3760),-4);
    }
    #[test]
    fn t_3760_sen_test(){
        assert_eq!(test_sen_money_number(3760),-16);
    }
    #[test]
    fn t_3760_gosen_test(){
        assert_eq!(test_gosen_money_number(3760),-4);
    }
    // 3770
    #[test]
    fn t_3770_ju_test(){
        assert_eq!(test_ju_money_number(3770),20);
    }
    #[test]
    fn t_3770_goju_test(){
        assert_eq!(test_goju_money_number(3770),-2);
    }
    #[test]
    fn t_3770_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3770),2);
    }
    #[test]
    fn t_3770_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3770),-4);
    }
    #[test]
    fn t_3770_sen_test(){
        assert_eq!(test_sen_money_number(3770),-16);
    }
    #[test]
    fn t_3770_gosen_test(){
        assert_eq!(test_gosen_money_number(3770),-4);
    }
    // 3780
    #[test]
    fn t_3780_ju_test(){
        assert_eq!(test_ju_money_number(3780),10);
    }
    #[test]
    fn t_3780_goju_test(){
        assert_eq!(test_goju_money_number(3780),-2);
    }
    #[test]
    fn t_3780_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3780),2);
    }
    #[test]
    fn t_3780_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3780),-4);
    }
    #[test]
    fn t_3780_sen_test(){
        assert_eq!(test_sen_money_number(3780),-16);
    }
    #[test]
    fn t_3780_gosen_test(){
        assert_eq!(test_gosen_money_number(3780),-4);
    }
    // 3790
    #[test]
    fn t_3790_ju_test(){
        assert_eq!(test_ju_money_number(3790),0);
    }
    #[test]
    fn t_3790_goju_test(){
        assert_eq!(test_goju_money_number(3790),-2);
    }
    #[test]
    fn t_3790_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3790),2);
    }
    #[test]
    fn t_3790_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3790),-4);
    }
    #[test]
    fn t_3790_sen_test(){
        assert_eq!(test_sen_money_number(3790),-16);
    }
    #[test]
    fn t_3790_gosen_test(){
        assert_eq!(test_gosen_money_number(3790),-4);
    }
    // 3800
    #[test]
    fn t_3800_ju_test(){
        assert_eq!(test_ju_money_number(3800),0);
    }
    #[test]
    fn t_3800_goju_test(){
        assert_eq!(test_goju_money_number(3800),0);
    }
    #[test]
    fn t_3800_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3800),5);
    }
    #[test]
    fn t_3800_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3800),-3);
    }
    #[test]
    fn t_3800_sen_test(){
        assert_eq!(test_sen_money_number(3800),-17);
    }
    #[test]
    fn t_3800_gosen_test(){
        assert_eq!(test_gosen_money_number(3800),-4);
    }
    // 3810
    #[test]
    fn t_3810_ju_test(){
        assert_eq!(test_ju_money_number(3810),30);
    }
    #[test]
    fn t_3810_goju_test(){
        assert_eq!(test_goju_money_number(3810),4);
    }
    #[test]
    fn t_3810_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3810),-16);
    }
    #[test]
    fn t_3810_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3810),-6);
    }
    #[test]
    fn t_3810_sen_test(){
        assert_eq!(test_sen_money_number(3810),-24);
    }
    #[test]
    fn t_3810_gosen_test(){
        assert_eq!(test_gosen_money_number(3810),-2);
    }
    // 3820
    #[test]
    fn t_3820_ju_test(){
        assert_eq!(test_ju_money_number(3820),20);
    }
    #[test]
    fn t_3820_goju_test(){
        assert_eq!(test_goju_money_number(3820),4);
    }
    #[test]
    fn t_3820_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3820),-16);
    }
    #[test]
    fn t_3820_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3820),-6);
    }
    #[test]
    fn t_3820_sen_test(){
        assert_eq!(test_sen_money_number(3820),-24);
    }
    #[test]
    fn t_3820_gosen_test(){
        assert_eq!(test_gosen_money_number(3820),-2);
    }
    // 3830
    #[test]
    fn t_3830_ju_test(){
        assert_eq!(test_ju_money_number(3830),10);
    }
    #[test]
    fn t_3830_goju_test(){
        assert_eq!(test_goju_money_number(3830),4);
    }
    #[test]
    fn t_3830_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3830),-16);
    }
    #[test]
    fn t_3830_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3830),-6);
    }
    #[test]
    fn t_3830_sen_test(){
        assert_eq!(test_sen_money_number(3830),-24);
    }
    #[test]
    fn t_3830_gosen_test(){
        assert_eq!(test_gosen_money_number(3830),-2);
    }
    // 3840
    #[test]
    fn t_3840_ju_test(){
        assert_eq!(test_ju_money_number(3840),0);
    }
    #[test]
    fn t_3840_goju_test(){
        assert_eq!(test_goju_money_number(3840),4);
    }
    #[test]
    fn t_3840_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3840),-16);
    }
    #[test]
    fn t_3840_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3840),-6);
    }
    #[test]
    fn t_3840_sen_test(){
        assert_eq!(test_sen_money_number(3840),-24);
    }
    #[test]
    fn t_3840_gosen_test(){
        assert_eq!(test_gosen_money_number(3840),-2);
    }
    // 3850
    #[test]
    fn t_3850_ju_test(){
        assert_eq!(test_ju_money_number(3850),0);
    }
    #[test]
    fn t_3850_goju_test(){
        assert_eq!(test_goju_money_number(3850),6);
    }
    #[test]
    fn t_3850_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3850),-8);
    }
    #[test]
    fn t_3850_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3850),-4);
    }
    #[test]
    fn t_3850_sen_test(){
        assert_eq!(test_sen_money_number(3850),-16);
    }
    #[test]
    fn t_3850_gosen_test(){
        assert_eq!(test_gosen_money_number(3850),-4);
    }
    // 3860
    #[test]
    fn t_3860_ju_test(){
        assert_eq!(test_ju_money_number(3860),30);
    }
    #[test]
    fn t_3860_goju_test(){
        assert_eq!(test_goju_money_number(3860),-2);
    }
    #[test]
    fn t_3860_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3860),-8);
    }
    #[test]
    fn t_3860_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3860),-4);
    }
    #[test]
    fn t_3860_sen_test(){
        assert_eq!(test_sen_money_number(3860),-16);
    }
    #[test]
    fn t_3860_gosen_test(){
        assert_eq!(test_gosen_money_number(3860),-4);
    }
    // 3870
    #[test]
    fn t_3870_ju_test(){
        assert_eq!(test_ju_money_number(3870),20);
    }
    #[test]
    fn t_3870_goju_test(){
        assert_eq!(test_goju_money_number(3870),-2);
    }
    #[test]
    fn t_3870_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3870),-8);
    }
    #[test]
    fn t_3870_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3870),-4);
    }
    #[test]
    fn t_3870_sen_test(){
        assert_eq!(test_sen_money_number(3870),-16);
    }
    #[test]
    fn t_3870_gosen_test(){
        assert_eq!(test_gosen_money_number(3870),-4);
    }
    // 3880
    #[test]
    fn t_3880_ju_test(){
        assert_eq!(test_ju_money_number(3880),10);
    }
    #[test]
    fn t_3880_goju_test(){
        assert_eq!(test_goju_money_number(3880),-2);
    }
    #[test]
    fn t_3880_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3880),-8);
    }
    #[test]
    fn t_3880_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3880),-4);
    }
    #[test]
    fn t_3880_sen_test(){
        assert_eq!(test_sen_money_number(3880),-16);
    }
    #[test]
    fn t_3880_gosen_test(){
        assert_eq!(test_gosen_money_number(3880),-4);
    }
    // 3890
    #[test]
    fn t_3890_ju_test(){
        assert_eq!(test_ju_money_number(3890),0);
    }
    #[test]
    fn t_3890_goju_test(){
        assert_eq!(test_goju_money_number(3890),-2);
    }
    #[test]
    fn t_3890_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3890),-8);
    }
    #[test]
    fn t_3890_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3890),-4);
    }
    #[test]
    fn t_3890_sen_test(){
        assert_eq!(test_sen_money_number(3890),-16);
    }
    #[test]
    fn t_3890_gosen_test(){
        assert_eq!(test_gosen_money_number(3890),-4);
    }
    // 3900
    #[test]
    fn t_3900_ju_test(){
        assert_eq!(test_ju_money_number(3900),0);
    }
    #[test]
    fn t_3900_goju_test(){
        assert_eq!(test_goju_money_number(3900),0);
    }
    #[test]
    fn t_3900_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3900),-5);
    }
    #[test]
    fn t_3900_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3900),-3);
    }
    #[test]
    fn t_3900_sen_test(){
        assert_eq!(test_sen_money_number(3900),-17);
    }
    #[test]
    fn t_3900_gosen_test(){
        assert_eq!(test_gosen_money_number(3900),-4);
    }
    // 3910
    #[test]
    fn t_3910_ju_test(){
        assert_eq!(test_ju_money_number(3910),30);
    }
    #[test]
    fn t_3910_goju_test(){
        assert_eq!(test_goju_money_number(3910),4);
    }
    #[test]
    fn t_3910_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3910),-16);
    }
    #[test]
    fn t_3910_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3910),-4);
    }
    #[test]
    fn t_3910_sen_test(){
        assert_eq!(test_sen_money_number(3910),-16);
    }
    #[test]
    fn t_3910_gosen_test(){
        assert_eq!(test_gosen_money_number(3910),-4);
    }
    // 3920
    #[test]
    fn t_3920_ju_test(){
        assert_eq!(test_ju_money_number(3920),20);
    }
    #[test]
    fn t_3920_goju_test(){
        assert_eq!(test_goju_money_number(3920),4);
    }
    #[test]
    fn t_3920_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3920),-16);
    }
    #[test]
    fn t_3920_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3920),-4);
    }
    #[test]
    fn t_3920_sen_test(){
        assert_eq!(test_sen_money_number(3920),-16);
    }
    #[test]
    fn t_3920_gosen_test(){
        assert_eq!(test_gosen_money_number(3920),-4);
    }
    // 3930
    #[test]
    fn t_3930_ju_test(){
        assert_eq!(test_ju_money_number(3930),10);
    }
    #[test]
    fn t_3930_goju_test(){
        assert_eq!(test_goju_money_number(3930),4);
    }
    #[test]
    fn t_3930_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3930),-16);
    }
    #[test]
    fn t_3930_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3930),-4);
    }
    #[test]
    fn t_3930_sen_test(){
        assert_eq!(test_sen_money_number(3930),-16);
    }
    #[test]
    fn t_3930_gosen_test(){
        assert_eq!(test_gosen_money_number(3930),-4);
    }
    // 3940
    #[test]
    fn t_3940_ju_test(){
        assert_eq!(test_ju_money_number(3940),0);
    }
    #[test]
    fn t_3940_goju_test(){
        assert_eq!(test_goju_money_number(3940),4);
    }
    #[test]
    fn t_3940_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3940),-16);
    }
    #[test]
    fn t_3940_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3940),-4);
    }
    #[test]
    fn t_3940_sen_test(){
        assert_eq!(test_sen_money_number(3940),-16);
    }
    #[test]
    fn t_3940_gosen_test(){
        assert_eq!(test_gosen_money_number(3940),-4);
    }
    // 3950
    #[test]
    fn t_3950_ju_test(){
        assert_eq!(test_ju_money_number(3950),0);
    }
    #[test]
    fn t_3950_goju_test(){
        assert_eq!(test_goju_money_number(3950),4);
    }
    #[test]
    fn t_3950_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3950),-12);
    }
    #[test]
    fn t_3950_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3950),-3);
    }
    #[test]
    fn t_3950_sen_test(){
        assert_eq!(test_sen_money_number(3950),-17);
    }
    #[test]
    fn t_3950_gosen_test(){
        assert_eq!(test_gosen_money_number(3950),-4);
    }
    // 3960
    #[test]
    fn t_3960_ju_test(){
        assert_eq!(test_ju_money_number(3960),25);
    }
    #[test]
    fn t_3960_goju_test(){
        assert_eq!(test_goju_money_number(3960),-3);
    }
    #[test]
    fn t_3960_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3960),-12);
    }
    #[test]
    fn t_3960_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3960),-3);
    }
    #[test]
    fn t_3960_sen_test(){
        assert_eq!(test_sen_money_number(3960),-17);
    }
    #[test]
    fn t_3960_gosen_test(){
        assert_eq!(test_gosen_money_number(3960),-4);
    }
    // 3970
    #[test]
    fn t_3970_ju_test(){
        assert_eq!(test_ju_money_number(3970),15);
    }
    #[test]
    fn t_3970_goju_test(){
        assert_eq!(test_goju_money_number(3970),-3);
    }
    #[test]
    fn t_3970_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3970),-12);
    }
    #[test]
    fn t_3970_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3970),-3);
    }
    #[test]
    fn t_3970_sen_test(){
        assert_eq!(test_sen_money_number(3970),-17);
    }
    #[test]
    fn t_3970_gosen_test(){
        assert_eq!(test_gosen_money_number(3970),-4);
    }
    // 3980
    #[test]
    fn t_3980_ju_test(){
        assert_eq!(test_ju_money_number(3980),5);
    }
    #[test]
    fn t_3980_goju_test(){
        assert_eq!(test_goju_money_number(3980),-3);
    }
    #[test]
    fn t_3980_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3980),-12);
    }
    #[test]
    fn t_3980_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3980),-3);
    }
    #[test]
    fn t_3980_sen_test(){
        assert_eq!(test_sen_money_number(3980),-17);
    }
    #[test]
    fn t_3980_gosen_test(){
        assert_eq!(test_gosen_money_number(3980),-4);
    }
    // 3990
    #[test]
    fn t_3990_ju_test(){
        assert_eq!(test_ju_money_number(3990),-5);
    }
    #[test]
    fn t_3990_goju_test(){
        assert_eq!(test_goju_money_number(3990),-3);
    }
    #[test]
    fn t_3990_hyaku_test(){
        assert_eq!(test_hyaku_money_number(3990),-12);
    }
    #[test]
    fn t_3990_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(3990),-3);
    }
    #[test]
    fn t_3990_sen_test(){
        assert_eq!(test_sen_money_number(3990),-17);
    }
    #[test]
    fn t_3990_gosen_test(){
        assert_eq!(test_gosen_money_number(3990),-4);
    }
    // 4000
    #[test]
    fn t_4000_ju_test(){
        assert_eq!(test_ju_money_number(4000),0);
    }
    #[test]
    fn t_4000_goju_test(){
        assert_eq!(test_goju_money_number(4000),0);
    }
    #[test]
    fn t_4000_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4000),0);
    }
    #[test]
    fn t_4000_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4000),0);
    }
    #[test]
    fn t_4000_sen_test(){
        assert_eq!(test_sen_money_number(4000),-15);
    }
    #[test]
    fn t_4000_gosen_test(){
        assert_eq!(test_gosen_money_number(4000),-5);
    }
    // 4010
    #[test]
    fn t_4010_ju_test(){
        assert_eq!(test_ju_money_number(4010),30);
    }
    #[test]
    fn t_4010_goju_test(){
        assert_eq!(test_goju_money_number(4010),4);
    }
    #[test]
    fn t_4010_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4010),14);
    }
    #[test]
    fn t_4010_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4010),0);
    }
    #[test]
    fn t_4010_sen_test(){
        assert_eq!(test_sen_money_number(4010),-32);
    }
    #[test]
    fn t_4010_gosen_test(){
        assert_eq!(test_gosen_money_number(4010),-2);
    }
    // 4020
    #[test]
    fn t_4020_ju_test(){
        assert_eq!(test_ju_money_number(4020),20);
    }
    #[test]
    fn t_4020_goju_test(){
        assert_eq!(test_goju_money_number(4020),4);
    }
    #[test]
    fn t_4020_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4020),14);
    }
    #[test]
    fn t_4020_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4020),0);
    }
    #[test]
    fn t_4020_sen_test(){
        assert_eq!(test_sen_money_number(4020),-32);
    }
    #[test]
    fn t_4020_gosen_test(){
        assert_eq!(test_gosen_money_number(4020),-2);
    }
    // 4030
    #[test]
    fn t_4030_ju_test(){
        assert_eq!(test_ju_money_number(4030),10);
    }
    #[test]
    fn t_4030_goju_test(){
        assert_eq!(test_goju_money_number(4030),4);
    }
    #[test]
    fn t_4030_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4030),14);
    }
    #[test]
    fn t_4030_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4030),0);
    }
    #[test]
    fn t_4030_sen_test(){
        assert_eq!(test_sen_money_number(4030),-32);
    }
    #[test]
    fn t_4030_gosen_test(){
        assert_eq!(test_gosen_money_number(4030),-2);
    }
    // 4040
    #[test]
    fn t_4040_ju_test(){
        assert_eq!(test_ju_money_number(4040),0);
    }
    #[test]
    fn t_4040_goju_test(){
        assert_eq!(test_goju_money_number(4040),4);
    }
    #[test]
    fn t_4040_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4040),14);
    }
    #[test]
    fn t_4040_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4040),0);
    }
    #[test]
    fn t_4040_sen_test(){
        assert_eq!(test_sen_money_number(4040),-32);
    }
    #[test]
    fn t_4040_gosen_test(){
        assert_eq!(test_gosen_money_number(4040),-2);
    }
    // 4050
    #[test]
    fn t_4050_ju_test(){
        assert_eq!(test_ju_money_number(4050),0);
    }
    #[test]
    fn t_4050_goju_test(){
        assert_eq!(test_goju_money_number(4050),6);
    }
    #[test]
    fn t_4050_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4050),22);
    }
    #[test]
    fn t_4050_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4050),2);
    }
    #[test]
    fn t_4050_sen_test(){
        assert_eq!(test_sen_money_number(4050),-24);
    }
    #[test]
    fn t_4050_gosen_test(){
        assert_eq!(test_gosen_money_number(4050),-4);
    }
    // 4060
    #[test]
    fn t_4060_ju_test(){
        assert_eq!(test_ju_money_number(4060),30);
    }
    #[test]
    fn t_4060_goju_test(){
        assert_eq!(test_goju_money_number(4060),-2);
    }
    #[test]
    fn t_4060_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4060),22);
    }
    #[test]
    fn t_4060_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4060),2);
    }
    #[test]
    fn t_4060_sen_test(){
        assert_eq!(test_sen_money_number(4060),-24);
    }
    #[test]
    fn t_4060_gosen_test(){
        assert_eq!(test_gosen_money_number(4060),-4);
    }
    // 4070
    #[test]
    fn t_4070_ju_test(){
        assert_eq!(test_ju_money_number(4070),20);
    }
    #[test]
    fn t_4070_goju_test(){
        assert_eq!(test_goju_money_number(4070),-2);
    }
    #[test]
    fn t_4070_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4070),22);
    }
    #[test]
    fn t_4070_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4070),2);
    }
    #[test]
    fn t_4070_sen_test(){
        assert_eq!(test_sen_money_number(4070),-24);
    }
    #[test]
    fn t_4070_gosen_test(){
        assert_eq!(test_gosen_money_number(4070),-4);
    }
    // 4080
    #[test]
    fn t_4080_ju_test(){
        assert_eq!(test_ju_money_number(4080),10);
    }
    #[test]
    fn t_4080_goju_test(){
        assert_eq!(test_goju_money_number(4080),-2);
    }
    #[test]
    fn t_4080_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4080),22);
    }
    #[test]
    fn t_4080_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4080),2);
    }
    #[test]
    fn t_4080_sen_test(){
        assert_eq!(test_sen_money_number(4080),-24);
    }
    #[test]
    fn t_4080_gosen_test(){
        assert_eq!(test_gosen_money_number(4080),-4);
    }
    // 4090
    #[test]
    fn t_4090_ju_test(){
        assert_eq!(test_ju_money_number(4090),0);
    }
    #[test]
    fn t_4090_goju_test(){
        assert_eq!(test_goju_money_number(4090),-2);
    }
    #[test]
    fn t_4090_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4090),22);
    }
    #[test]
    fn t_4090_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4090),2);
    }
    #[test]
    fn t_4090_sen_test(){
        assert_eq!(test_sen_money_number(4090),-24);
    }
    #[test]
    fn t_4090_gosen_test(){
        assert_eq!(test_gosen_money_number(4090),-4);
    }
    // 4100
    #[test]
    fn t_4100_ju_test(){
        assert_eq!(test_ju_money_number(4100),0);
    }
    #[test]
    fn t_4100_goju_test(){
        assert_eq!(test_goju_money_number(4100),0);
    }
    #[test]
    fn t_4100_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4100),25);
    }
    #[test]
    fn t_4100_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4100),1);
    }
    #[test]
    fn t_4100_sen_test(){
        assert_eq!(test_sen_money_number(4100),-24);
    }
    // 4110
    #[test]
    fn t_4110_ju_test(){
        assert_eq!(test_ju_money_number(4110),30);
    }
    #[test]
    fn t_4110_goju_test(){
        assert_eq!(test_goju_money_number(4110),4);
    }
    #[test]
    fn t_4110_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4110),4);
    }
    #[test]
    fn t_4110_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4110),0);
    }
    #[test]
    fn t_4110_sen_test(){
        assert_eq!(test_sen_money_number(4110),-32);
    }
    #[test]
    fn t_4110_gosen_test(){
        assert_eq!(test_gosen_money_number(4110),-2);
    }
    // 4120
    #[test]
    fn t_4120_ju_test(){
        assert_eq!(test_ju_money_number(4120),20);
    }
    #[test]
    fn t_4120_goju_test(){
        assert_eq!(test_goju_money_number(4120),4);
    }
    #[test]
    fn t_4120_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4120),4);
    }
    #[test]
    fn t_4120_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4120),0);
    }
    #[test]
    fn t_4120_sen_test(){
        assert_eq!(test_sen_money_number(4120),-32);
    }
    #[test]
    fn t_4120_gosen_test(){
        assert_eq!(test_gosen_money_number(4120),-2);
    }
    // 4130
    #[test]
    fn t_4130_ju_test(){
        assert_eq!(test_ju_money_number(4130),10);
    }
    #[test]
    fn t_4130_goju_test(){
        assert_eq!(test_goju_money_number(4130),4);
    }
    #[test]
    fn t_4130_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4130),4);
    }
    #[test]
    fn t_4130_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4130),0);
    }
    #[test]
    fn t_4130_sen_test(){
        assert_eq!(test_sen_money_number(4130),-32);
    }
    #[test]
    fn t_4130_gosen_test(){
        assert_eq!(test_gosen_money_number(4130),-2);
    }
    // 4140
    #[test]
    fn t_4140_ju_test(){
        assert_eq!(test_ju_money_number(4140),0);
    }
    #[test]
    fn t_4140_goju_test(){
        assert_eq!(test_goju_money_number(4140),4);
    }
    #[test]
    fn t_4140_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4140),4);
    }
    #[test]
    fn t_4140_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4140),0);
    }
    #[test]
    fn t_4140_sen_test(){
        assert_eq!(test_sen_money_number(4140),-32);
    }
    #[test]
    fn t_4140_gosen_test(){
        assert_eq!(test_gosen_money_number(4140),-2);
    }
    // 4150
    #[test]
    fn t_4150_ju_test(){
        assert_eq!(test_ju_money_number(4150),0);
    }
    #[test]
    fn t_4150_goju_test(){
        assert_eq!(test_goju_money_number(4150),6);
    }
    #[test]
    fn t_4150_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4150),12);
    }
    #[test]
    fn t_4150_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4150),2);
    }
    #[test]
    fn t_4150_sen_test(){
        assert_eq!(test_sen_money_number(4150),-24);
    }
    #[test]
    fn t_4150_gosen_test(){
        assert_eq!(test_gosen_money_number(4150),-4);
    }
    // 4160
    #[test]
    fn t_4160_ju_test(){
        assert_eq!(test_ju_money_number(4160),30);
    }
    #[test]
    fn t_4160_goju_test(){
        assert_eq!(test_goju_money_number(4160),-2);
    }
    #[test]
    fn t_4160_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4160),12);
    }
    #[test]
    fn t_4160_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4160),2);
    }
    #[test]
    fn t_4160_sen_test(){
        assert_eq!(test_sen_money_number(4160),-24);
    }
    #[test]
    fn t_4160_gosen_test(){
        assert_eq!(test_gosen_money_number(4160),-4);
    }
    // 4170
    #[test]
    fn t_4170_ju_test(){
        assert_eq!(test_ju_money_number(4170),20);
    }
    #[test]
    fn t_4170_goju_test(){
        assert_eq!(test_goju_money_number(4170),-2);
    }
    #[test]
    fn t_4170_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4170),12);
    }
    #[test]
    fn t_4170_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4170),2);
    }
    #[test]
    fn t_4170_sen_test(){
        assert_eq!(test_sen_money_number(4170),-24);
    }
    #[test]
    fn t_4170_gosen_test(){
        assert_eq!(test_gosen_money_number(4170),-4);
    }
    // 4180
    #[test]
    fn t_4180_ju_test(){
        assert_eq!(test_ju_money_number(4180),10);
    }
    #[test]
    fn t_4180_goju_test(){
        assert_eq!(test_goju_money_number(4180),-2);
    }
    #[test]
    fn t_4180_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4180),12);
    }
    #[test]
    fn t_4180_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4180),2);
    }
    #[test]
    fn t_4180_sen_test(){
        assert_eq!(test_sen_money_number(4180),-24);
    }
    #[test]
    fn t_4180_gosen_test(){
        assert_eq!(test_gosen_money_number(4180),-4);
    }
    // 4190
    #[test]
    fn t_4190_ju_test(){
        assert_eq!(test_ju_money_number(4190),0);
    }
    #[test]
    fn t_4190_goju_test(){
        assert_eq!(test_goju_money_number(4190),-2);
    }
    #[test]
    fn t_4190_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4190),12);
    }
    #[test]
    fn t_4190_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4190),2);
    }
    #[test]
    fn t_4190_sen_test(){
        assert_eq!(test_sen_money_number(4190),-24);
    }
    #[test]
    fn t_4190_gosen_test(){
        assert_eq!(test_gosen_money_number(4190),-4);
    }
    // 4200
    #[test]
    fn t_4200_ju_test(){
        assert_eq!(test_ju_money_number(4200),0);
    }
    #[test]
    fn t_4200_goju_test(){
        assert_eq!(test_goju_money_number(4200),0);
    }
    #[test]
    fn t_4200_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4200),15);
    }
    #[test]
    fn t_4200_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4200),1);
    }
    #[test]
    fn t_4200_sen_test(){
        assert_eq!(test_sen_money_number(4200),-24);
    }
    #[test]
    fn t_4200_gosen_test(){
        assert_eq!(test_gosen_money_number(4200),-4);
    }
    // 4210
    #[test]
    fn t_4210_ju_test(){
        assert_eq!(test_ju_money_number(4210),30);
    }
    #[test]
    fn t_4210_goju_test(){
        assert_eq!(test_goju_money_number(4210),4);
    }
    #[test]
    fn t_4210_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4210),-6);
    }
    #[test]
    fn t_4210_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4210),0);
    }
    #[test]
    fn t_4210_sen_test(){
        assert_eq!(test_sen_money_number(4210),-32);
    }
    #[test]
    fn t_4210_gosen_test(){
        assert_eq!(test_gosen_money_number(4210),-2);
    }
    // 4220
    #[test]
    fn t_4220_ju_test(){
        assert_eq!(test_ju_money_number(4220),20);
    }
    #[test]
    fn t_4220_goju_test(){
        assert_eq!(test_goju_money_number(4220),4);
    }
    #[test]
    fn t_4220_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4220),-6);
    }
    #[test]
    fn t_4220_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4220),0);
    }
    #[test]
    fn t_4220_sen_test(){
        assert_eq!(test_sen_money_number(4220),-32);
    }
    #[test]
    fn t_4220_gosen_test(){
        assert_eq!(test_gosen_money_number(4220),-2);
    }
    // 4230
    #[test]
    fn t_4230_ju_test(){
        assert_eq!(test_ju_money_number(4230),10);
    }
    #[test]
    fn t_4230_goju_test(){
        assert_eq!(test_goju_money_number(4230),4);
    }
    #[test]
    fn t_4230_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4230),-6);
    }
    #[test]
    fn t_4230_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4230),0);
    }
    #[test]
    fn t_4230_sen_test(){
        assert_eq!(test_sen_money_number(4230),-32);
    }
    #[test]
    fn t_4230_gosen_test(){
        assert_eq!(test_gosen_money_number(4230),-2);
    }
    // 4240
    #[test]
    fn t_4240_ju_test(){
        assert_eq!(test_ju_money_number(4240),0);
    }
    #[test]
    fn t_4240_goju_test(){
        assert_eq!(test_goju_money_number(4240),4);
    }
    #[test]
    fn t_4240_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4240),-6);
    }
    #[test]
    fn t_4240_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4240),0);
    }
    #[test]
    fn t_4240_sen_test(){
        assert_eq!(test_sen_money_number(4240),-32);
    }
    #[test]
    fn t_4240_gosen_test(){
        assert_eq!(test_gosen_money_number(4240),-2);
    }
    // 4250
    #[test]
    fn t_4250_ju_test(){
        assert_eq!(test_ju_money_number(4250),0);
    }
    #[test]
    fn t_4250_goju_test(){
        assert_eq!(test_goju_money_number(4250),6);
    }
    #[test]
    fn t_4250_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4250),2);
    }
    #[test]
    fn t_4250_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4250),2);
    }
    #[test]
    fn t_4250_sen_test(){
        assert_eq!(test_sen_money_number(4250),-24);
    }
    #[test]
    fn t_4250_gosen_test(){
        assert_eq!(test_gosen_money_number(4250),-4);
    }
    // 4260
    #[test]
    fn t_4260_ju_test(){
        assert_eq!(test_ju_money_number(4260),30);
    }
    #[test]
    fn t_4260_goju_test(){
        assert_eq!(test_goju_money_number(4260),-2);
    }
    #[test]
    fn t_4260_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4260),2);
    }
    #[test]
    fn t_4260_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4260),2);
    }
    #[test]
    fn t_4260_sen_test(){
        assert_eq!(test_sen_money_number(4260),-24);
    }
    #[test]
    fn t_4260_gosen_test(){
        assert_eq!(test_gosen_money_number(4260),-4);
    }
    // 4270
    #[test]
    fn t_4270_ju_test(){
        assert_eq!(test_ju_money_number(4270),20);
    }
    #[test]
    fn t_4270_goju_test(){
        assert_eq!(test_goju_money_number(4270),-2);
    }
    #[test]
    fn t_4270_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4270),2);
    }
    #[test]
    fn t_4270_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4270),2);
    }
    #[test]
    fn t_4270_sen_test(){
        assert_eq!(test_sen_money_number(4270),-24);
    }
    #[test]
    fn t_4270_gosen_test(){
        assert_eq!(test_gosen_money_number(4270),-4);
    }
    // 4280
    #[test]
    fn t_4280_ju_test(){
        assert_eq!(test_ju_money_number(4280),10);
    }
    #[test]
    fn t_4280_goju_test(){
        assert_eq!(test_goju_money_number(4280),-2);
    }
    #[test]
    fn t_4280_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4280),2);
    }
    #[test]
    fn t_4280_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4280),2);
    }
    #[test]
    fn t_4280_sen_test(){
        assert_eq!(test_sen_money_number(4280),-24);
    }
    #[test]
    fn t_4280_gosen_test(){
        assert_eq!(test_gosen_money_number(4280),-4);
    }
    // 4290
    #[test]
    fn t_4290_ju_test(){
        assert_eq!(test_ju_money_number(4290),0);
    }
    #[test]
    fn t_4290_goju_test(){
        assert_eq!(test_goju_money_number(4290),-2);
    }
    #[test]
    fn t_4290_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4290),2);
    }
    #[test]
    fn t_4290_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4290),2);
    }
    #[test]
    fn t_4290_sen_test(){
        assert_eq!(test_sen_money_number(4290),-24);
    }
    #[test]
    fn t_4290_gosen_test(){
        assert_eq!(test_gosen_money_number(4290),-4);
    }
    // 4300
    #[test]
    fn t_4300_ju_test(){
        assert_eq!(test_ju_money_number(4300),0);
    }
    #[test]
    fn t_4300_goju_test(){
        assert_eq!(test_goju_money_number(4300),0);
    }
    #[test]
    fn t_4300_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4300),5);
    }
    #[test]
    fn t_4300_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4300),1);
    }
    #[test]
    fn t_4300_sen_test(){
        assert_eq!(test_sen_money_number(4300),-24);
    }
    #[test]
    fn t_4300_gosen_test(){
        assert_eq!(test_gosen_money_number(4300),-4);
    }
    // 4310
    #[test]
    fn t_4310_ju_test(){
        assert_eq!(test_ju_money_number(4310),30);
    }
    #[test]
    fn t_4310_goju_test(){
        assert_eq!(test_goju_money_number(4310),4);
    }
    #[test]
    fn t_4310_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4310),-16);
    }
    #[test]
    fn t_4310_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4310),0);
    }
    #[test]
    fn t_4310_sen_test(){
        assert_eq!(test_sen_money_number(4310),-32);
    }
    #[test]
    fn t_4310_gosen_test(){
        assert_eq!(test_gosen_money_number(4310),-2);
    }
    // 4320
    #[test]
    fn t_4320_ju_test(){
        assert_eq!(test_ju_money_number(4320),20);
    }
    #[test]
    fn t_4320_goju_test(){
        assert_eq!(test_goju_money_number(4320),4);
    }
    #[test]
    fn t_4320_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4320),-16);
    }
    #[test]
    fn t_4320_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4320),0);
    }
    #[test]
    fn t_4320_sen_test(){
        assert_eq!(test_sen_money_number(4320),-32);
    }
    #[test]
    fn t_4320_gosen_test(){
        assert_eq!(test_gosen_money_number(4320),-2);
    }
    // 4330
    #[test]
    fn t_4330_ju_test(){
        assert_eq!(test_ju_money_number(4330),10);
    }
    #[test]
    fn t_4330_goju_test(){
        assert_eq!(test_goju_money_number(4330),4);
    }
    #[test]
    fn t_4330_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4330),-16);
    }
    #[test]
    fn t_4330_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4330),0);
    }
    #[test]
    fn t_4330_sen_test(){
        assert_eq!(test_sen_money_number(4330),-32);
    }
    #[test]
    fn t_4330_gosen_test(){
        assert_eq!(test_gosen_money_number(4330),-2);
    }
    // 4340
    #[test]
    fn t_4340_ju_test(){
        assert_eq!(test_ju_money_number(4340),0);
    }
    #[test]
    fn t_4340_goju_test(){
        assert_eq!(test_goju_money_number(4340),4);
    }
    #[test]
    fn t_4340_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4340),-16);
    }
    #[test]
    fn t_4340_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4340),0);
    }
    #[test]
    fn t_4340_sen_test(){
        assert_eq!(test_sen_money_number(4340),-32);
    }
    #[test]
    fn t_4340_gosen_test(){
        assert_eq!(test_gosen_money_number(4340),-2);
    }
    // 4350
    #[test]
    fn t_4350_ju_test(){
        assert_eq!(test_ju_money_number(4350),0);
    }
    #[test]
    fn t_4350_goju_test(){
        assert_eq!(test_goju_money_number(4350),6);
    }
    #[test]
    fn t_4350_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4350),-8);
    }
    #[test]
    fn t_4350_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4350),2);
    }
    #[test]
    fn t_4350_sen_test(){
        assert_eq!(test_sen_money_number(4350),-24);
    }
    #[test]
    fn t_4350_gosen_test(){
        assert_eq!(test_gosen_money_number(4350),-4);
    }
    // 4360
    #[test]
    fn t_4360_ju_test(){
        assert_eq!(test_ju_money_number(4360),30);
    }
    #[test]
    fn t_4360_goju_test(){
        assert_eq!(test_goju_money_number(4360),-2);
    }
    #[test]
    fn t_4360_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4360),-8);
    }
    #[test]
    fn t_4360_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4360),2);
    }
    #[test]
    fn t_4360_sen_test(){
        assert_eq!(test_sen_money_number(4360),-24);
    }
    #[test]
    fn t_4360_gosen_test(){
        assert_eq!(test_gosen_money_number(4360),-4);
    }
    // 4370
    #[test]
    fn t_4370_ju_test(){
        assert_eq!(test_ju_money_number(4370),20);
    }
    #[test]
    fn t_4370_goju_test(){
        assert_eq!(test_goju_money_number(4370),-2);
    }
    #[test]
    fn t_4370_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4370),-8);
    }
    #[test]
    fn t_4370_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4370),2);
    }
    #[test]
    fn t_4370_sen_test(){
        assert_eq!(test_sen_money_number(4370),-24);
    }
    #[test]
    fn t_4370_gosen_test(){
        assert_eq!(test_gosen_money_number(4370),-4);
    }
    // 4380
    #[test]
    fn t_4380_ju_test(){
        assert_eq!(test_ju_money_number(4380),10);
    }
    #[test]
    fn t_4380_goju_test(){
        assert_eq!(test_goju_money_number(4380),-2);
    }
    #[test]
    fn t_4380_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4380),-8);
    }
    #[test]
    fn t_4380_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4380),2);
    }
    #[test]
    fn t_4380_sen_test(){
        assert_eq!(test_sen_money_number(4380),-24);
    }
    #[test]
    fn t_4380_gosen_test(){
        assert_eq!(test_gosen_money_number(4380),-4);
    }
    // 4390
    #[test]
    fn t_4390_ju_test(){
        assert_eq!(test_ju_money_number(4390),0);
    }
    #[test]
    fn t_4390_goju_test(){
        assert_eq!(test_goju_money_number(4390),-2);
    }
    #[test]
    fn t_4390_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4390),-8);
    }
    #[test]
    fn t_4390_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4390),2);
    }
    #[test]
    fn t_4390_sen_test(){
        assert_eq!(test_sen_money_number(4390),-24);
    }
    #[test]
    fn t_4390_gosen_test(){
        assert_eq!(test_gosen_money_number(4390),-4);
    }
    // 4400
    #[test]
    fn t_4400_ju_test(){
        assert_eq!(test_ju_money_number(4400),0);
    }
    #[test]
    fn t_4400_goju_test(){
        assert_eq!(test_goju_money_number(4400),0);
    }
    #[test]
    fn t_4400_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4400),-5);
    }
    #[test]
    fn t_4400_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4400),1);
    }
    #[test]
    fn t_4400_sen_test(){
        assert_eq!(test_sen_money_number(4400),-24);
    }
    #[test]
    fn t_4400_gosen_test(){
        assert_eq!(test_gosen_money_number(4400),-4);
    }
    // 4410
    #[test]
    fn t_4410_ju_test(){
        assert_eq!(test_ju_money_number(4410),30);
    }
    #[test]
    fn t_4410_goju_test(){
        assert_eq!(test_goju_money_number(4410),4);
    }
    #[test]
    fn t_4410_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4410),-16);
    }
    #[test]
    fn t_4410_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4410),2);
    }
    #[test]
    fn t_4410_sen_test(){
        assert_eq!(test_sen_money_number(4410),-24);
    }
    #[test]
    fn t_4410_gosen_test(){
        assert_eq!(test_gosen_money_number(4410),-4);
    }
    // 4420
    #[test]
    fn t_4420_ju_test(){
        assert_eq!(test_ju_money_number(4420),20);
    }
    #[test]
    fn t_4420_goju_test(){
        assert_eq!(test_goju_money_number(4420),4);
    }
    #[test]
    fn t_4420_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4420),-16);
    }
    #[test]
    fn t_4420_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4420),2);
    }
    #[test]
    fn t_4420_sen_test(){
        assert_eq!(test_sen_money_number(4420),-24);
    }
    #[test]
    fn t_4420_gosen_test(){
        assert_eq!(test_gosen_money_number(4420),-4);
    }
    // 4430
    #[test]
    fn t_4430_ju_test(){
        assert_eq!(test_ju_money_number(4430),10);
    }
    #[test]
    fn t_4430_goju_test(){
        assert_eq!(test_goju_money_number(4430),4);
    }
    #[test]
    fn t_4430_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4430),-16);
    }
    #[test]
    fn t_4430_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4430),2);
    }
    #[test]
    fn t_4430_sen_test(){
        assert_eq!(test_sen_money_number(4430),-24);
    }
    #[test]
    fn t_4430_gosen_test(){
        assert_eq!(test_gosen_money_number(4430),-4);
    }
    // 4440
    #[test]
    fn t_4440_ju_test(){
        assert_eq!(test_ju_money_number(4440),0);
    }
    #[test]
    fn t_4440_goju_test(){
        assert_eq!(test_goju_money_number(4440),4);
    }
    #[test]
    fn t_4440_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4440),-16);
    }
    #[test]
    fn t_4440_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4440),2);
    }
    #[test]
    fn t_4440_sen_test(){
        assert_eq!(test_sen_money_number(4440),-24);
    }
    #[test]
    fn t_4440_gosen_test(){
        assert_eq!(test_gosen_money_number(4440),-4);
    }
    // 4450
    #[test]
    fn t_4450_ju_test(){
        assert_eq!(test_ju_money_number(4450),0);
    }
    #[test]
    fn t_4450_goju_test(){
        assert_eq!(test_goju_money_number(4450),4);
    }
    #[test]
    fn t_4450_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4450),-12);
    }
    #[test]
    fn t_4450_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4450),1);
    }
    #[test]
    fn t_4450_sen_test(){
        assert_eq!(test_sen_money_number(4450),-24);
    }
    #[test]
    fn t_4450_gosen_test(){
        assert_eq!(test_gosen_money_number(4450),-4);
    }
    // 4460
    #[test]
    fn t_4460_ju_test(){
        assert_eq!(test_ju_money_number(4460),25);
    }
    #[test]
    fn t_4460_goju_test(){
        assert_eq!(test_goju_money_number(4460),-3);
    }
    #[test]
    fn t_4460_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4460),-12);
    }
    #[test]
    fn t_4460_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4460),1);
    }
    #[test]
    fn t_4460_sen_test(){
        assert_eq!(test_sen_money_number(4460),-24);
    }
    #[test]
    fn t_4460_gosen_test(){
        assert_eq!(test_gosen_money_number(4460),-4);
    }
    // 4470
    #[test]
    fn t_4470_ju_test(){
        assert_eq!(test_ju_money_number(4470),15);
    }
    #[test]
    fn t_4470_goju_test(){
        assert_eq!(test_goju_money_number(4470),-3);
    }
    #[test]
    fn t_4470_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4470),-12);
    }
    #[test]
    fn t_4470_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4470),1);
    }
    #[test]
    fn t_4470_sen_test(){
        assert_eq!(test_sen_money_number(4470),-24);
    }
    #[test]
    fn t_4470_gosen_test(){
        assert_eq!(test_gosen_money_number(4470),-4);
    }
    // 4480
    #[test]
    fn t_4480_ju_test(){
        assert_eq!(test_ju_money_number(4480),5);
    }
    #[test]
    fn t_4480_goju_test(){
        assert_eq!(test_goju_money_number(4480),-3);
    }
    #[test]
    fn t_4480_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4480),-12);
    }
    #[test]
    fn t_4480_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4480),1);
    }
    #[test]
    fn t_4480_sen_test(){
        assert_eq!(test_sen_money_number(4480),-24);
    }
    #[test]
    fn t_4480_gosen_test(){
        assert_eq!(test_gosen_money_number(4480),-4);
    }
    // 4490
    #[test]
    fn t_4490_ju_test(){
        assert_eq!(test_ju_money_number(4490),-5);
    }
    #[test]
    fn t_4490_goju_test(){
        assert_eq!(test_goju_money_number(4490),-3);
    }
    #[test]
    fn t_4490_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4490),-12);
    }
    #[test]
    fn t_4490_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4490),1);
    }
    #[test]
    fn t_4490_sen_test(){
        assert_eq!(test_sen_money_number(4490),-24);
    }
    #[test]
    fn t_4490_gosen_test(){
        assert_eq!(test_gosen_money_number(4490),-4);
    }
    // 4500
    #[test]
    fn t_4500_ju_test(){
        assert_eq!(test_ju_money_number(4500),0);
    }
    #[test]
    fn t_4500_goju_test(){
        assert_eq!(test_goju_money_number(4500),0);
    }
    #[test]
    fn t_4500_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4500),0);
    }
    #[test]
    fn t_4500_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4500),0);
    }
    #[test]
    fn t_4500_sen_test(){
        assert_eq!(test_sen_money_number(4500),-20);
    }
    #[test]
    fn t_4500_gosen_test(){
        assert_eq!(test_gosen_money_number(4500),-5);
    }
    // 4510
    #[test]
    fn t_4510_ju_test(){
        assert_eq!(test_ju_money_number(4510),30);
    }
    #[test]
    fn t_4510_goju_test(){
        assert_eq!(test_goju_money_number(4510),4);
    }
    #[test]
    fn t_4510_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4510),14);
    }
    #[test]
    fn t_4510_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4510),-6);
    }
    #[test]
    fn t_4510_sen_test(){
        assert_eq!(test_sen_money_number(4510),-24);
    }
    #[test]
    fn t_4510_gosen_test(){
        assert_eq!(test_gosen_money_number(4510),-4);
    }
    // 4520
    #[test]
    fn t_4520_ju_test(){
        assert_eq!(test_ju_money_number(4520),20);
    }
    #[test]
    fn t_4520_goju_test(){
        assert_eq!(test_goju_money_number(4520),4);
    }
    #[test]
    fn t_4520_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4520),14);
    }
    #[test]
    fn t_4520_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4520),-6);
    }
    #[test]
    fn t_4520_sen_test(){
        assert_eq!(test_sen_money_number(4520),-24);
    }
    #[test]
    fn t_4520_gosen_test(){
        assert_eq!(test_gosen_money_number(4520),-4);
    }
    // 4530
    #[test]
    fn t_4530_ju_test(){
        assert_eq!(test_ju_money_number(4530),10);
    }
    #[test]
    fn t_4530_goju_test(){
        assert_eq!(test_goju_money_number(4530),4);
    }
    #[test]
    fn t_4530_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4530),14);
    }
    #[test]
    fn t_4530_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4530),-6);
    }
    #[test]
    fn t_4530_sen_test(){
        assert_eq!(test_sen_money_number(4530),-24);
    }
    #[test]
    fn t_4530_gosen_test(){
        assert_eq!(test_gosen_money_number(4530),-4);
    }
    // 4540
    #[test]
    fn t_4540_ju_test(){
        assert_eq!(test_ju_money_number(4540),0);
    }
    #[test]
    fn t_4540_goju_test(){
        assert_eq!(test_goju_money_number(4540),4);
    }
    #[test]
    fn t_4540_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4540),14);
    }
    #[test]
    fn t_4540_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4540),-6);
    }
    #[test]
    fn t_4540_sen_test(){
        assert_eq!(test_sen_money_number(4540),-24);
    }
    #[test]
    fn t_4540_gosen_test(){
        assert_eq!(test_gosen_money_number(4540),-4);
    }
    // 4550
    #[test]
    fn t_4550_ju_test(){
        assert_eq!(test_ju_money_number(4550),0);
    }
    #[test]
    fn t_4550_goju_test(){
        assert_eq!(test_goju_money_number(4550),4);
    }
    #[test]
    fn t_4550_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4550),13);
    }
    #[test]
    fn t_4550_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4550),-6);
    }
    #[test]
    fn t_4550_sen_test(){
        assert_eq!(test_sen_money_number(4550),-24);
    }
    #[test]
    fn t_4550_gosen_test(){
        assert_eq!(test_gosen_money_number(4550),-4);
    }
    // 4560
    #[test]
    fn t_4560_ju_test(){
        assert_eq!(test_ju_money_number(4560),25);
    }
    #[test]
    fn t_4560_goju_test(){
        assert_eq!(test_goju_money_number(4560),-3);
    }
    #[test]
    fn t_4560_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4560),13);
    }
    #[test]
    fn t_4560_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4560),-6);
    }
    #[test]
    fn t_4560_sen_test(){
        assert_eq!(test_sen_money_number(4560),-24);
    }
    #[test]
    fn t_4560_gosen_test(){
        assert_eq!(test_gosen_money_number(4560),-4);
    }
    // 4570
    #[test]
    fn t_4570_ju_test(){
        assert_eq!(test_ju_money_number(4570),15);
    }
    #[test]
    fn t_4570_goju_test(){
        assert_eq!(test_goju_money_number(4570),-3);
    }
    #[test]
    fn t_4570_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4570),13);
    }
    #[test]
    fn t_4570_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4570),-6);
    }
    #[test]
    fn t_4570_sen_test(){
        assert_eq!(test_sen_money_number(4570),-24);
    }
    #[test]
    fn t_4570_gosen_test(){
        assert_eq!(test_gosen_money_number(4570),-4);
    }
    // 4580
    #[test]
    fn t_4580_ju_test(){
        assert_eq!(test_ju_money_number(4580),5);
    }
    #[test]
    fn t_4580_goju_test(){
        assert_eq!(test_goju_money_number(4580),-3);
    }
    #[test]
    fn t_4580_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4580),13);
    }
    #[test]
    fn t_4580_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4580),-6);
    }
    #[test]
    fn t_4580_sen_test(){
        assert_eq!(test_sen_money_number(4580),-24);
    }
    #[test]
    fn t_4580_gosen_test(){
        assert_eq!(test_gosen_money_number(4580),-4);
    }
    // 4590
    #[test]
    fn t_4590_ju_test(){
        assert_eq!(test_ju_money_number(4590),-5);
    }
    #[test]
    fn t_4590_goju_test(){
        assert_eq!(test_goju_money_number(4590),-3);
    }
    #[test]
    fn t_4590_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4590),13);
    }
    #[test]
    fn t_4590_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4590),-6);
    }
    #[test]
    fn t_4590_sen_test(){
        assert_eq!(test_sen_money_number(4590),-24);
    }
    #[test]
    fn t_4590_gosen_test(){
        assert_eq!(test_gosen_money_number(4590),-4);
    }
    // 4600
    #[test]
    fn t_4600_ju_test(){
        assert_eq!(test_ju_money_number(4600),0);
    }
    #[test]
    fn t_4600_goju_test(){
        assert_eq!(test_goju_money_number(4600),0);
    }
    #[test]
    fn t_4600_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4600),15);
    }
    #[test]
    fn t_4600_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4600),-5);
    }
    #[test]
    fn t_4600_sen_test(){
        assert_eq!(test_sen_money_number(4600),-20);
    }
    #[test]
    fn t_4600_gosen_test(){
        assert_eq!(test_gosen_money_number(4600),-5);
    }
    // 4610
    #[test]
    fn t_4610_ju_test(){
        assert_eq!(test_ju_money_number(4610),30);
    }
    #[test]
    fn t_4610_goju_test(){
        assert_eq!(test_goju_money_number(4610),4);
    }
    #[test]
    fn t_4610_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4610),4);
    }
    #[test]
    fn t_4610_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4610),-6);
    }
    #[test]
    fn t_4610_sen_test(){
        assert_eq!(test_sen_money_number(4610),-24);
    }
    #[test]
    fn t_4610_gosen_test(){
        assert_eq!(test_gosen_money_number(4610),-4);
    }
    // 4620
    #[test]
    fn t_4620_ju_test(){
        assert_eq!(test_ju_money_number(4620),20);
    }
    #[test]
    fn t_4620_goju_test(){
        assert_eq!(test_goju_money_number(4620),4);
    }
    #[test]
    fn t_4620_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4620),4);
    }
    #[test]
    fn t_4620_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4620),-6);
    }
    #[test]
    fn t_4620_sen_test(){
        assert_eq!(test_sen_money_number(4620),-24);
    }
    #[test]
    fn t_4620_gosen_test(){
        assert_eq!(test_gosen_money_number(4620),-4);
    }
    // 4630
    #[test]
    fn t_4630_ju_test(){
        assert_eq!(test_ju_money_number(4630),10);
    }
    #[test]
    fn t_4630_goju_test(){
        assert_eq!(test_goju_money_number(4630),4);
    }
    #[test]
    fn t_4630_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4630),4);
    }
    #[test]
    fn t_4630_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4630),-6);
    }
    #[test]
    fn t_4630_sen_test(){
        assert_eq!(test_sen_money_number(4630),-24);
    }
    #[test]
    fn t_4630_gosen_test(){
        assert_eq!(test_gosen_money_number(4630),-4);
    }
    // 4640
    #[test]
    fn t_4640_ju_test(){
        assert_eq!(test_ju_money_number(4640),0);
    }
    #[test]
    fn t_4640_goju_test(){
        assert_eq!(test_goju_money_number(4640),4);
    }
    #[test]
    fn t_4640_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4640),4);
    }
    #[test]
    fn t_4640_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4640),-6);
    }
    #[test]
    fn t_4640_sen_test(){
        assert_eq!(test_sen_money_number(4640),-24);
    }
    #[test]
    fn t_4640_gosen_test(){
        assert_eq!(test_gosen_money_number(4640),-4);
    }
    // 4650
    #[test]
    fn t_4650_ju_test(){
        assert_eq!(test_ju_money_number(4650),0);
    }
    #[test]
    fn t_4650_goju_test(){
        assert_eq!(test_goju_money_number(4650),4);
    }
    #[test]
    fn t_4650_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4650),3);
    }
    #[test]
    fn t_4650_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4650),-6);
    }
    #[test]
    fn t_4650_sen_test(){
        assert_eq!(test_sen_money_number(4650),-24);
    }
    #[test]
    fn t_4650_gosen_test(){
        assert_eq!(test_gosen_money_number(4650),-4);
    }
    // 4660
    #[test]
    fn t_4660_ju_test(){
        assert_eq!(test_ju_money_number(4660),25);
    }
    #[test]
    fn t_4660_goju_test(){
        assert_eq!(test_goju_money_number(4660),-3);
    }
    #[test]
    fn t_4660_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4660),3);
    }
    #[test]
    fn t_4660_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4660),-6);
    }
    #[test]
    fn t_4660_sen_test(){
        assert_eq!(test_sen_money_number(4660),-24);
    }
    #[test]
    fn t_4660_gosen_test(){
        assert_eq!(test_gosen_money_number(4660),-4);
    }
    // 4670
    #[test]
    fn t_4670_ju_test(){
        assert_eq!(test_ju_money_number(4670),15);
    }
    #[test]
    fn t_4670_goju_test(){
        assert_eq!(test_goju_money_number(4670),-3);
    }
    #[test]
    fn t_4670_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4670),3);
    }
    #[test]
    fn t_4670_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4670),-6);
    }
    #[test]
    fn t_4670_sen_test(){
        assert_eq!(test_sen_money_number(4670),-24);
    }
    #[test]
    fn t_4670_gosen_test(){
        assert_eq!(test_gosen_money_number(4670),-4);
    }
    // 4680
    #[test]
    fn t_4680_ju_test(){
        assert_eq!(test_ju_money_number(4680),5);
    }
    #[test]
    fn t_4680_goju_test(){
        assert_eq!(test_goju_money_number(4680),-3);
    }
    #[test]
    fn t_4680_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4680),3);
    }
    #[test]
    fn t_4680_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4680),-6);
    }
    #[test]
    fn t_4680_sen_test(){
        assert_eq!(test_sen_money_number(4680),-24);
    }
    #[test]
    fn t_4680_gosen_test(){
        assert_eq!(test_gosen_money_number(4680),-4);
    }
    // 4690
    #[test]
    fn t_4690_ju_test(){
        assert_eq!(test_ju_money_number(4690),-5);
    }
    #[test]
    fn t_4690_goju_test(){
        assert_eq!(test_goju_money_number(4690),-3);
    }
    #[test]
    fn t_4690_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4690),3);
    }
    #[test]
    fn t_4690_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4690),-6);
    }
    #[test]
    fn t_4690_sen_test(){
        assert_eq!(test_sen_money_number(4690),-24);
    }
    #[test]
    fn t_4690_gosen_test(){
        assert_eq!(test_gosen_money_number(4690),-4);
    }
    // 4700
    #[test]
    fn t_4700_ju_test(){
        assert_eq!(test_ju_money_number(4700),0);
    }
    #[test]
    fn t_4700_goju_test(){
        assert_eq!(test_goju_money_number(4700),0);
    }
    #[test]
    fn t_4700_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4700),5);
    }
    #[test]
    fn t_4700_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4700),-5);
    }
    #[test]
    fn t_4700_sen_test(){
        assert_eq!(test_sen_money_number(4700),-20);
    }
    #[test]
    fn t_4700_gosen_test(){
        assert_eq!(test_gosen_money_number(4700),-5);
    }
    // 4710
    #[test]
    fn t_4710_ju_test(){
        assert_eq!(test_ju_money_number(4710),30);
    }
    #[test]
    fn t_4710_goju_test(){
        assert_eq!(test_goju_money_number(4710),4);
    }
    #[test]
    fn t_4710_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4710),-6);
    }
    #[test]
    fn t_4710_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4710),-6);
    }
    #[test]
    fn t_4710_sen_test(){
        assert_eq!(test_sen_money_number(4710),-24);
    }
    #[test]
    fn t_4710_gosen_test(){
        assert_eq!(test_gosen_money_number(4710),-4);
    }
    // 4720
    #[test]
    fn t_4720_ju_test(){
        assert_eq!(test_ju_money_number(4720),20);
    }
    #[test]
    fn t_4720_goju_test(){
        assert_eq!(test_goju_money_number(4720),4);
    }
    #[test]
    fn t_4720_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4720),-6);
    }
    #[test]
    fn t_4720_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4720),-6);
    }
    #[test]
    fn t_4720_sen_test(){
        assert_eq!(test_sen_money_number(4720),-24);
    }
    #[test]
    fn t_4720_gosen_test(){
        assert_eq!(test_gosen_money_number(4720),-4);
    }
    // 4730
    #[test]
    fn t_4730_ju_test(){
        assert_eq!(test_ju_money_number(4730),10);
    }
    #[test]
    fn t_4730_goju_test(){
        assert_eq!(test_goju_money_number(4730),4);
    }
    #[test]
    fn t_4730_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4730),-6);
    }
    #[test]
    fn t_4730_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4730),-6);
    }
    #[test]
    fn t_4730_sen_test(){
        assert_eq!(test_sen_money_number(4730),-24);
    }
    #[test]
    fn t_4730_gosen_test(){
        assert_eq!(test_gosen_money_number(4730),-4);
    }
    // 4740
    #[test]
    fn t_4740_ju_test(){
        assert_eq!(test_ju_money_number(4740),0);
    }
    #[test]
    fn t_4740_goju_test(){
        assert_eq!(test_goju_money_number(4740),4);
    }
    #[test]
    fn t_4740_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4740),-6);
    }
    #[test]
    fn t_4740_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4740),-6);
    }
    #[test]
    fn t_4740_sen_test(){
        assert_eq!(test_sen_money_number(4740),-24);
    }
    #[test]
    fn t_4740_gosen_test(){
        assert_eq!(test_gosen_money_number(4740),-4);
    }
    // 4750
    #[test]
    fn t_4750_ju_test(){
        assert_eq!(test_ju_money_number(4750),0);
    }
    #[test]
    fn t_4750_goju_test(){
        assert_eq!(test_goju_money_number(4750),4);
    }
    #[test]
    fn t_4750_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4750),-7);
    }
    #[test]
    fn t_4750_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4750),-6);
    }
    #[test]
    fn t_4750_sen_test(){
        assert_eq!(test_sen_money_number(4750),-24);
    }
    #[test]
    fn t_4750_gosen_test(){
        assert_eq!(test_gosen_money_number(4750),-4);
    }
    // 4760
    #[test]
    fn t_4760_ju_test(){
        assert_eq!(test_ju_money_number(4760),25);
    }
    #[test]
    fn t_4760_goju_test(){
        assert_eq!(test_goju_money_number(4760),-3);
    }
    #[test]
    fn t_4760_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4760),-7);
    }
    #[test]
    fn t_4760_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4760),-6);
    }
    #[test]
    fn t_4760_sen_test(){
        assert_eq!(test_sen_money_number(4760),-24);
    }
    #[test]
    fn t_4760_gosen_test(){
        assert_eq!(test_gosen_money_number(4760),-4);
    }
    // 4770
    #[test]
    fn t_4770_ju_test(){
        assert_eq!(test_ju_money_number(4770),15);
    }
    #[test]
    fn t_4770_goju_test(){
        assert_eq!(test_goju_money_number(4770),-3);
    }
    #[test]
    fn t_4770_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4770),-7);
    }
    #[test]
    fn t_4770_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4770),-6);
    }
    #[test]
    fn t_4770_sen_test(){
        assert_eq!(test_sen_money_number(4770),-24);
    }
    #[test]
    fn t_4770_gosen_test(){
        assert_eq!(test_gosen_money_number(4770),-4);
    }
    // 4780
    #[test]
    fn t_4780_ju_test(){
        assert_eq!(test_ju_money_number(4780),5);
    }
    #[test]
    fn t_4780_goju_test(){
        assert_eq!(test_goju_money_number(4780),-3);
    }
    #[test]
    fn t_4780_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4780),-7);
    }
    #[test]
    fn t_4780_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4780),-6);
    }
    #[test]
    fn t_4780_sen_test(){
        assert_eq!(test_sen_money_number(4780),-24);
    }
    #[test]
    fn t_4780_gosen_test(){
        assert_eq!(test_gosen_money_number(4780),-4);
    }
    // 4790
    #[test]
    fn t_4790_ju_test(){
        assert_eq!(test_ju_money_number(4790),-5);
    }
    #[test]
    fn t_4790_goju_test(){
        assert_eq!(test_goju_money_number(4790),-3);
    }
    #[test]
    fn t_4790_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4790),-7);
    }
    #[test]
    fn t_4790_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4790),-6);
    }
    #[test]
    fn t_4790_sen_test(){
        assert_eq!(test_sen_money_number(4790),-24);
    }
    #[test]
    fn t_4790_gosen_test(){
        assert_eq!(test_gosen_money_number(4790),-4);
    }
    // 4800
    #[test]
    fn t_4800_ju_test(){
        assert_eq!(test_ju_money_number(4800),0);
    }
    #[test]
    fn t_4800_goju_test(){
        assert_eq!(test_goju_money_number(4800),0);
    }
    #[test]
    fn t_4800_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4800),-5);
    }
    #[test]
    fn t_4800_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4800),-5);
    }
    #[test]
    fn t_4800_sen_test(){
        assert_eq!(test_sen_money_number(4800),-20);
    }
    #[test]
    fn t_4800_gosen_test(){
        assert_eq!(test_gosen_money_number(4800),-5);
    }
    // 4810
    #[test]
    fn t_4810_ju_test(){
        assert_eq!(test_ju_money_number(4810),30);
    }
    #[test]
    fn t_4810_goju_test(){
        assert_eq!(test_goju_money_number(4810),4);
    }
    #[test]
    fn t_4810_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4810),-16);
    }
    #[test]
    fn t_4810_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4810),-6);
    }
    #[test]
    fn t_4810_sen_test(){
        assert_eq!(test_sen_money_number(4810),-24);
    }
    #[test]
    fn t_4810_gosen_test(){
        assert_eq!(test_gosen_money_number(4810),-4);
    }
    // 4820
    #[test]
    fn t_4820_ju_test(){
        assert_eq!(test_ju_money_number(4820),20);
    }
    #[test]
    fn t_4820_goju_test(){
        assert_eq!(test_goju_money_number(4820),4);
    }
    #[test]
    fn t_4820_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4820),-16);
    }
    #[test]
    fn t_4820_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4820),-6);
    }
    #[test]
    fn t_4820_sen_test(){
        assert_eq!(test_sen_money_number(4820),-24);
    }
    #[test]
    fn t_4820_gosen_test(){
        assert_eq!(test_gosen_money_number(4820),-4);
    }
    // 4830
    #[test]
    fn t_4830_ju_test(){
        assert_eq!(test_ju_money_number(4830),10);
    }
    #[test]
    fn t_4830_goju_test(){
        assert_eq!(test_goju_money_number(4830),4);
    }
    #[test]
    fn t_4830_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4830),-16);
    }
    #[test]
    fn t_4830_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4830),-6);
    }
    #[test]
    fn t_4830_sen_test(){
        assert_eq!(test_sen_money_number(4830),-24);
    }
    #[test]
    fn t_4830_gosen_test(){
        assert_eq!(test_gosen_money_number(4830),-4);
    }
    // 4840
    #[test]
    fn t_4840_ju_test(){
        assert_eq!(test_ju_money_number(4840),0);
    }
    #[test]
    fn t_4840_goju_test(){
        assert_eq!(test_goju_money_number(4840),4);
    }
    #[test]
    fn t_4840_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4840),-16);
    }
    #[test]
    fn t_4840_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4840),-6);
    }
    #[test]
    fn t_4840_sen_test(){
        assert_eq!(test_sen_money_number(4840),-24);
    }
    #[test]
    fn t_4840_gosen_test(){
        assert_eq!(test_gosen_money_number(4840),-4);
    }
    // 4850
    #[test]
    fn t_4850_ju_test(){
        assert_eq!(test_ju_money_number(4850),0);
    }
    #[test]
    fn t_4850_goju_test(){
        assert_eq!(test_goju_money_number(4850),4);
    }
    #[test]
    fn t_4850_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4850),-17);
    }
    #[test]
    fn t_4850_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4850),-6);
    }
    #[test]
    fn t_4850_sen_test(){
        assert_eq!(test_sen_money_number(4850),-24);
    }
    #[test]
    fn t_4850_gosen_test(){
        assert_eq!(test_gosen_money_number(4850),-4);
    }
    // 4860
    #[test]
    fn t_4860_ju_test(){
        assert_eq!(test_ju_money_number(4860),25);
    }
    #[test]
    fn t_4860_goju_test(){
        assert_eq!(test_goju_money_number(4860),-3);
    }
    #[test]
    fn t_4860_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4860),-17);
    }
    #[test]
    fn t_4860_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4860),-6);
    }
    #[test]
    fn t_4860_sen_test(){
        assert_eq!(test_sen_money_number(4860),-24);
    }
    #[test]
    fn t_4860_gosen_test(){
        assert_eq!(test_gosen_money_number(4860),-4);
    }
    // 4870
    #[test]
    fn t_4870_ju_test(){
        assert_eq!(test_ju_money_number(4870),15);
    }
    #[test]
    fn t_4870_goju_test(){
        assert_eq!(test_goju_money_number(4870),-3);
    }
    #[test]
    fn t_4870_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4870),-17);
    }
    #[test]
    fn t_4870_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4870),-6);
    }
    #[test]
    fn t_4870_sen_test(){
        assert_eq!(test_sen_money_number(4870),-24);
    }
    #[test]
    fn t_4870_gosen_test(){
        assert_eq!(test_gosen_money_number(4870),-4);
    }
    // 4880
    #[test]
    fn t_4880_ju_test(){
        assert_eq!(test_ju_money_number(4880),5);
    }
    #[test]
    fn t_4880_goju_test(){
        assert_eq!(test_goju_money_number(4880),-3);
    }
    #[test]
    fn t_4880_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4880),-17);
    }
    #[test]
    fn t_4880_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4880),-6);
    }
    #[test]
    fn t_4880_sen_test(){
        assert_eq!(test_sen_money_number(4880),-24);
    }
    #[test]
    fn t_4880_gosen_test(){
        assert_eq!(test_gosen_money_number(4880),-4);
    }
    // 4890
    #[test]
    fn t_4890_ju_test(){
        assert_eq!(test_ju_money_number(4890),-5);
    }
    #[test]
    fn t_4890_goju_test(){
        assert_eq!(test_goju_money_number(4890),-3);
    }
    #[test]
    fn t_4890_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4890),-17);
    }
    #[test]
    fn t_4890_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4890),-6);
    }
    #[test]
    fn t_4890_sen_test(){
        assert_eq!(test_sen_money_number(4890),-24);
    }
    #[test]
    fn t_4890_gosen_test(){
        assert_eq!(test_gosen_money_number(4890),-4);
    }
    // 4900
    #[test]
    fn t_4900_ju_test(){
        assert_eq!(test_ju_money_number(4900),0);
    }
    #[test]
    fn t_4900_goju_test(){
        assert_eq!(test_goju_money_number(4900),0);
    }
    #[test]
    fn t_4900_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4900),-15);
    }
    #[test]
    fn t_4900_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4900),-5);
    }
    #[test]
    fn t_4900_sen_test(){
        assert_eq!(test_sen_money_number(4900),-20);
    }
    #[test]
    fn t_4900_gosen_test(){
        assert_eq!(test_gosen_money_number(4900),-5);
    }
    // 4910
    #[test]
    fn t_4910_ju_test(){
        assert_eq!(test_ju_money_number(4910),25);
    }
    #[test]
    fn t_4910_goju_test(){
        assert_eq!(test_goju_money_number(4910),1);
    }
    #[test]
    fn t_4910_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4910),-24);
    }
    #[test]
    fn t_4910_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4910),-6);
    }
    #[test]
    fn t_4910_sen_test(){
        assert_eq!(test_sen_money_number(4910),-24);
    }
    #[test]
    fn t_4910_gosen_test(){
        assert_eq!(test_gosen_money_number(4910),-4);
    }
    // 4920
    #[test]
    fn t_4920_ju_test(){
        assert_eq!(test_ju_money_number(4920),15);
    }
    #[test]
    fn t_4920_goju_test(){
        assert_eq!(test_goju_money_number(4920),1);
    }
    #[test]
    fn t_4920_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4920),-24);
    }
    #[test]
    fn t_4920_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4920),-6);
    }
    #[test]
    fn t_4920_sen_test(){
        assert_eq!(test_sen_money_number(4920),-24);
    }
    #[test]
    fn t_4920_gosen_test(){
        assert_eq!(test_gosen_money_number(4920),-4);
    }
    // 4930
    #[test]
    fn t_4930_ju_test(){
        assert_eq!(test_ju_money_number(4930),5);
    }
    #[test]
    fn t_4930_goju_test(){
        assert_eq!(test_goju_money_number(4930),1);
    }
    #[test]
    fn t_4930_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4930),-24);
    }
    #[test]
    fn t_4930_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4930),-6);
    }
    #[test]
    fn t_4930_sen_test(){
        assert_eq!(test_sen_money_number(4930),-24);
    }
    #[test]
    fn t_4930_gosen_test(){
        assert_eq!(test_gosen_money_number(4930),-4);
    }
    // 4940
    #[test]
    fn t_4940_ju_test(){
        assert_eq!(test_ju_money_number(4940),-5);
    }
    #[test]
    fn t_4940_goju_test(){
        assert_eq!(test_goju_money_number(4940),1);
    }
    #[test]
    fn t_4940_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4940),-24);
    }
    #[test]
    fn t_4940_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4940),-6);
    }
    #[test]
    fn t_4940_sen_test(){
        assert_eq!(test_sen_money_number(4940),-24);
    }
    #[test]
    fn t_4940_gosen_test(){
        assert_eq!(test_gosen_money_number(4940),-4);
    }
    // 4950
    #[test]
    fn t_4950_ju_test(){
        assert_eq!(test_ju_money_number(4950),0);
    }
    #[test]
    fn t_4950_goju_test(){
        assert_eq!(test_goju_money_number(4950),0);
    }
    #[test]
    fn t_4950_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4950),-20);
    }
    #[test]
    fn t_4950_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4950),-5);
    }
    #[test]
    fn t_4950_sen_test(){
        assert_eq!(test_sen_money_number(4950),-20);
    }
    #[test]
    fn t_4950_gosen_test(){
        assert_eq!(test_gosen_money_number(4950),-5);
    }
    // 4960
    #[test]
    fn t_4960_ju_test(){
        assert_eq!(test_ju_money_number(4960),15);
    }
    #[test]
    fn t_4960_goju_test(){
        assert_eq!(test_goju_money_number(4960),-5);
    }
    #[test]
    fn t_4960_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4960),-20);
    }
    #[test]
    fn t_4960_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4960),-5);
    }
    #[test]
    fn t_4960_sen_test(){
        assert_eq!(test_sen_money_number(4960),-20);
    }
    #[test]
    fn t_4960_gosen_test(){
        assert_eq!(test_gosen_money_number(4960),-5);
    }
    // 4970
    #[test]
    fn t_4970_ju_test(){
        assert_eq!(test_ju_money_number(4970),5);
    }
    #[test]
    fn t_4970_goju_test(){
        assert_eq!(test_goju_money_number(4970),-5);
    }
    #[test]
    fn t_4970_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4970),-20);
    }
    #[test]
    fn t_4970_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4970),-5);
    }
    #[test]
    fn t_4970_sen_test(){
        assert_eq!(test_sen_money_number(4970),-20);
    }
    #[test]
    fn t_4970_gosen_test(){
        assert_eq!(test_gosen_money_number(4970),-5);
    }
    // 4980
    #[test]
    fn t_4980_ju_test(){
        assert_eq!(test_ju_money_number(4980),-5);
    }
    #[test]
    fn t_4980_goju_test(){
        assert_eq!(test_goju_money_number(4980),-5);
    }
    #[test]
    fn t_4980_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4980),-20);
    }
    #[test]
    fn t_4980_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4980),-5);
    }
    #[test]
    fn t_4980_sen_test(){
        assert_eq!(test_sen_money_number(4980),-20);
    }
    #[test]
    fn t_4980_gosen_test(){
        assert_eq!(test_gosen_money_number(4980),-5);
    }
    // 4990
    #[test]
    fn t_4990_ju_test(){
        assert_eq!(test_ju_money_number(4990),-15);
    }
    #[test]
    fn t_4990_goju_test(){
        assert_eq!(test_goju_money_number(4990),-5);
    }
    #[test]
    fn t_4990_hyaku_test(){
        assert_eq!(test_hyaku_money_number(4990),-20);
    }
    #[test]
    fn t_4990_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(4990),-5);
    }
    #[test]
    fn t_4990_sen_test(){
        assert_eq!(test_sen_money_number(4990),-20);
    }
    #[test]
    fn t_4990_gosen_test(){
        assert_eq!(test_gosen_money_number(4990),-5);
    }
    // 5000
    #[test]
    fn t_5000_ju_test(){
        assert_eq!(test_ju_money_number(5000),0);
    }
    #[test]
    fn t_5000_goju_test(){
        assert_eq!(test_goju_money_number(5000),0);
    }
    #[test]
    fn t_5000_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5000),0);
    }
    #[test]
    fn t_5000_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5000),0);
    }
    #[test]
    fn t_5000_sen_test(){
        assert_eq!(test_sen_money_number(5000),0);
    }
    #[test]
    fn t_5000_gosen_test(){
        assert_eq!(test_gosen_money_number(5000),0);
    }
    #[test]
    fn t_5000_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5000),-5);
    }
    // 5010
    #[test]
    fn t_5010_ju_test(){
        assert_eq!(test_ju_money_number(5010),35);
    }
    #[test]
    fn t_5010_goju_test(){
        assert_eq!(test_goju_money_number(5010),7);
    }
    #[test]
    fn t_5010_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5010),27);
    }
    #[test]
    fn t_5010_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5010),5);
    }
    #[test]
    fn t_5010_sen_test(){
        assert_eq!(test_sen_money_number(5010),19);
    }
    #[test]
    fn t_5010_gosen_test(){
        assert_eq!(test_gosen_money_number(5010),-5);
    }
    #[test]
    fn t_5010_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5010),-5);
    }
    // 5020
    #[test]
    fn t_5020_ju_test(){
        assert_eq!(test_ju_money_number(5020),25);
    }
    #[test]
    fn t_5020_goju_test(){
        assert_eq!(test_goju_money_number(5020),7);
    }
    #[test]
    fn t_5020_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5020),27);
    }
    #[test]
    fn t_5020_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5020),5);
    }
    #[test]
    fn t_5020_sen_test(){
        assert_eq!(test_sen_money_number(5020),19);
    }
    #[test]
    fn t_5020_gosen_test(){
        assert_eq!(test_gosen_money_number(5020),-5);
    }
    #[test]
    fn t_5020_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5020),-5);
    }
    // 5030
    #[test]
    fn t_5030_ju_test(){
        assert_eq!(test_ju_money_number(5030),15);
    }
    #[test]
    fn t_5030_goju_test(){
        assert_eq!(test_goju_money_number(5030),7);
    }
    #[test]
    fn t_5030_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5030),27);
    }
    #[test]
    fn t_5030_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5030),5);
    }
    #[test]
    fn t_5030_sen_test(){
        assert_eq!(test_sen_money_number(5030),19);
    }
    #[test]
    fn t_5030_gosen_test(){
        assert_eq!(test_gosen_money_number(5030),-5);
    }
    #[test]
    fn t_5030_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5030),-5);
    }
    // 5040
    #[test]
    fn t_5040_ju_test(){
        assert_eq!(test_ju_money_number(5040),5);
    }
    #[test]
    fn t_5040_goju_test(){
        assert_eq!(test_goju_money_number(5040),7);
    }
    #[test]
    fn t_5040_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5040),27);
    }
    #[test]
    fn t_5040_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5040),5);
    }
    #[test]
    fn t_5040_sen_test(){
        assert_eq!(test_sen_money_number(5040),19);
    }
    #[test]
    fn t_5040_gosen_test(){
        assert_eq!(test_gosen_money_number(5040),-5);
    }
    #[test]
    fn t_5040_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5040),-5);
    }
    // 5050
    #[test]
    fn t_5050_ju_test(){
        assert_eq!(test_ju_money_number(5050),0);
    }
    #[test]
    fn t_5050_goju_test(){
        assert_eq!(test_goju_money_number(5050),6);
    }
    #[test]
    fn t_5050_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5050),22);
    }
    #[test]
    fn t_5050_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5050),2);
    }
    #[test]
    fn t_5050_sen_test(){
        assert_eq!(test_sen_money_number(5050),6);
    }
    #[test]
    fn t_5050_gosen_test(){
        assert_eq!(test_gosen_money_number(5050),-8);
    }
    #[test]
    fn t_5050_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5050),-2);
    }
    // 5060
    #[test]
    fn t_5060_ju_test(){
        assert_eq!(test_ju_money_number(5060),30);
    }
    #[test]
    fn t_5060_goju_test(){
        assert_eq!(test_goju_money_number(5060),-2);
    }
    #[test]
    fn t_5060_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5060),22);
    }
    #[test]
    fn t_5060_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5060),2);
    }
    #[test]
    fn t_5060_sen_test(){
        assert_eq!(test_sen_money_number(5060),6);
    }
    #[test]
    fn t_5060_gosen_test(){
        assert_eq!(test_gosen_money_number(5060),-8);
    }
    #[test]
    fn t_5060_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5060),-2);
    }
    // 5070
    #[test]
    fn t_5070_ju_test(){
        assert_eq!(test_ju_money_number(5070),20);
    }
    #[test]
    fn t_5070_goju_test(){
        assert_eq!(test_goju_money_number(5070),-2);
    }
    #[test]
    fn t_5070_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5070),22);
    }
    #[test]
    fn t_5070_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5070),2);
    }
    #[test]
    fn t_5070_sen_test(){
        assert_eq!(test_sen_money_number(5070),6);
    }
    #[test]
    fn t_5070_gosen_test(){
        assert_eq!(test_gosen_money_number(5070),-8);
    }
    #[test]
    fn t_5070_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5070),-2);
    }
    // 5080
    #[test]
    fn t_5080_ju_test(){
        assert_eq!(test_ju_money_number(5080),10);
    }
    #[test]
    fn t_5080_goju_test(){
        assert_eq!(test_goju_money_number(5080),-2);
    }
    #[test]
    fn t_5080_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5080),22);
    }
    #[test]
    fn t_5080_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5080),2);
    }
    #[test]
    fn t_5080_sen_test(){
        assert_eq!(test_sen_money_number(5080),6);
    }
    #[test]
    fn t_5080_gosen_test(){
        assert_eq!(test_gosen_money_number(5080),-8);
    }
    #[test]
    fn t_5080_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5080),-2);
    }
    // 5090
    #[test]
    fn t_5090_ju_test(){
        assert_eq!(test_ju_money_number(5090),0);
    }
    #[test]
    fn t_5090_goju_test(){
        assert_eq!(test_goju_money_number(5090),-2);
    }
    #[test]
    fn t_5090_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5090),22);
    }
    #[test]
    fn t_5090_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5090),2);
    }
    #[test]
    fn t_5090_sen_test(){
        assert_eq!(test_sen_money_number(5090),6);
    }
    #[test]
    fn t_5090_gosen_test(){
        assert_eq!(test_gosen_money_number(5090),-8);
    }
    #[test]
    fn t_5090_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5090),-2);
    }
    // 5100
    #[test]
    fn t_5100_ju_test(){
        assert_eq!(test_ju_money_number(5100),0);
    }
    #[test]
    fn t_5100_goju_test(){
        assert_eq!(test_goju_money_number(5100),0);
    }
    #[test]
    fn t_5100_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5100),30);
    }
    #[test]
    fn t_5100_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5100),4);
    }
    #[test]
    fn t_5100_sen_test(){
        assert_eq!(test_sen_money_number(5100),14);
    }
    #[test]
    fn t_5100_gosen_test(){
        assert_eq!(test_gosen_money_number(5100),-6);
    }
    #[test]
    fn t_5100_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5100),-4);
    }
    // 5110
    #[test]
    fn t_5110_ju_test(){
        assert_eq!(test_ju_money_number(5110),35);
    }
    #[test]
    fn t_5110_goju_test(){
        assert_eq!(test_goju_money_number(5110),7);
    }
    #[test]
    fn t_5110_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5110),17);
    }
    #[test]
    fn t_5110_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5110),5);
    }
    #[test]
    fn t_5110_sen_test(){
        assert_eq!(test_sen_money_number(5110),19);
    }
    #[test]
    fn t_5110_gosen_test(){
        assert_eq!(test_gosen_money_number(5110),-5);
    }
    #[test]
    fn t_5110_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5110),-5);
    }
    // 5120
    #[test]
    fn t_5120_ju_test(){
        assert_eq!(test_ju_money_number(5120),25);
    }
    #[test]
    fn t_5120_goju_test(){
        assert_eq!(test_goju_money_number(5120),7);
    }
    #[test]
    fn t_5120_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5120),17);
    }
    #[test]
    fn t_5120_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5120),5);
    }
    #[test]
    fn t_5120_sen_test(){
        assert_eq!(test_sen_money_number(5120),19);
    }
    #[test]
    fn t_5120_gosen_test(){
        assert_eq!(test_gosen_money_number(5120),-5);
    }
    #[test]
    fn t_5120_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5120),-5);
    }
    // 5130
    #[test]
    fn t_5130_ju_test(){
        assert_eq!(test_ju_money_number(5130),15);
    }
    #[test]
    fn t_5130_goju_test(){
        assert_eq!(test_goju_money_number(5130),7);
    }
    #[test]
    fn t_5130_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5130),17);
    }
    #[test]
    fn t_5130_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5130),5);
    }
    #[test]
    fn t_5130_sen_test(){
        assert_eq!(test_sen_money_number(5130),19);
    }
    #[test]
    fn t_5130_gosen_test(){
        assert_eq!(test_gosen_money_number(5130),-5);
    }
    #[test]
    fn t_5130_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5130),-5);
    }
    // 5140
    #[test]
    fn t_5140_ju_test(){
        assert_eq!(test_ju_money_number(5140),5);
    }
    #[test]
    fn t_5140_goju_test(){
        assert_eq!(test_goju_money_number(5140),7);
    }
    #[test]
    fn t_5140_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5140),17);
    }
    #[test]
    fn t_5140_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5140),5);
    }
    #[test]
    fn t_5140_sen_test(){
        assert_eq!(test_sen_money_number(5140),19);
    }
    #[test]
    fn t_5140_gosen_test(){
        assert_eq!(test_gosen_money_number(5140),-5);
    }
    #[test]
    fn t_5140_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5140),-5);
    }
    // 5150
    #[test]
    fn t_5150_ju_test(){
        assert_eq!(test_ju_money_number(5150),0);
    }
    #[test]
    fn t_5150_goju_test(){
        assert_eq!(test_goju_money_number(5150),6);
    }
    #[test]
    fn t_5150_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5150),12);
    }
    #[test]
    fn t_5150_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5150),2);
    }
    #[test]
    fn t_5150_sen_test(){
        assert_eq!(test_sen_money_number(5150),6);
    }
    #[test]
    fn t_5150_gosen_test(){
        assert_eq!(test_gosen_money_number(5150),-8);
    }
    #[test]
    fn t_5150_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5150),-2);
    }
    // 5160
    #[test]
    fn t_5160_ju_test(){
        assert_eq!(test_ju_money_number(5160),30);
    }
    #[test]
    fn t_5160_goju_test(){
        assert_eq!(test_goju_money_number(5160),-2);
    }
    #[test]
    fn t_5160_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5160),12);
    }
    #[test]
    fn t_5160_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5160),2);
    }
    #[test]
    fn t_5160_sen_test(){
        assert_eq!(test_sen_money_number(5160),6);
    }
    #[test]
    fn t_5160_gosen_test(){
        assert_eq!(test_gosen_money_number(5160),-8);
    }
    #[test]
    fn t_5160_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5160),-2);
    }
    // 5170
    #[test]
    fn t_5170_ju_test(){
        assert_eq!(test_ju_money_number(5170),20);
    }
    #[test]
    fn t_5170_goju_test(){
        assert_eq!(test_goju_money_number(5170),-2);
    }
    #[test]
    fn t_5170_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5170),12);
    }
    #[test]
    fn t_5170_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5170),2);
    }
    #[test]
    fn t_5170_sen_test(){
        assert_eq!(test_sen_money_number(5170),6);
    }
    #[test]
    fn t_5170_gosen_test(){
        assert_eq!(test_gosen_money_number(5170),-8);
    }
    #[test]
    fn t_5170_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5170),-2);
    }
    // 5180
    #[test]
    fn t_5180_ju_test(){
        assert_eq!(test_ju_money_number(5180),10);
    }
    #[test]
    fn t_5180_goju_test(){
        assert_eq!(test_goju_money_number(5180),-2);
    }
    #[test]
    fn t_5180_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5180),12);
    }
    #[test]
    fn t_5180_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5180),2);
    }
    #[test]
    fn t_5180_sen_test(){
        assert_eq!(test_sen_money_number(5180),6);
    }
    #[test]
    fn t_5180_gosen_test(){
        assert_eq!(test_gosen_money_number(5180),-8);
    }
    #[test]
    fn t_5180_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5180),-2);
    }
    // 5190
    #[test]
    fn t_5190_ju_test(){
        assert_eq!(test_ju_money_number(5190),0);
    }
    #[test]
    fn t_5190_goju_test(){
        assert_eq!(test_goju_money_number(5190),-2);
    }
    #[test]
    fn t_5190_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5190),12);
    }
    #[test]
    fn t_5190_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5190),2);
    }
    #[test]
    fn t_5190_sen_test(){
        assert_eq!(test_sen_money_number(5190),6);
    }
    #[test]
    fn t_5190_gosen_test(){
        assert_eq!(test_gosen_money_number(5190),-8);
    }
    #[test]
    fn t_5190_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5190),-2);
    }
    // 5200
    #[test]
    fn t_5200_ju_test(){
        assert_eq!(test_ju_money_number(5200),0);
    }
    #[test]
    fn t_5200_goju_test(){
        assert_eq!(test_goju_money_number(5200),0);
    }
    #[test]
    fn t_5200_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5200),20);
    }
    #[test]
    fn t_5200_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5200),4);
    }
    #[test]
    fn t_5200_sen_test(){
        assert_eq!(test_sen_money_number(5200),14);
    }
    #[test]
    fn t_5200_gosen_test(){
        assert_eq!(test_gosen_money_number(5200),-6);
    }
    #[test]
    fn t_5200_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5200),-4);
    }
    // 5210
    #[test]
    fn t_5210_ju_test(){
        assert_eq!(test_ju_money_number(5210),35);
    }
    #[test]
    fn t_5210_goju_test(){
        assert_eq!(test_goju_money_number(5210),7);
    }
    #[test]
    fn t_5210_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5210),7);
    }
    #[test]
    fn t_5210_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5210),5);
    }
    #[test]
    fn t_5210_sen_test(){
        assert_eq!(test_sen_money_number(5210),19);
    }
    #[test]
    fn t_5210_gosen_test(){
        assert_eq!(test_gosen_money_number(5210),-5);
    }
    #[test]
    fn t_5210_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5210),-5);
    }
    // 5220
    #[test]
    fn t_5220_ju_test(){
        assert_eq!(test_ju_money_number(5220),25);
    }
    #[test]
    fn t_5220_goju_test(){
        assert_eq!(test_goju_money_number(5220),7);
    }
    #[test]
    fn t_5220_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5220),7);
    }
    #[test]
    fn t_5220_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5220),5);
    }
    #[test]
    fn t_5220_sen_test(){
        assert_eq!(test_sen_money_number(5220),19);
    }
    #[test]
    fn t_5220_gosen_test(){
        assert_eq!(test_gosen_money_number(5220),-5);
    }
    #[test]
    fn t_5220_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5220),-5);
    }
    // 5230
    #[test]
    fn t_5230_ju_test(){
        assert_eq!(test_ju_money_number(5230),15);
    }
    #[test]
    fn t_5230_goju_test(){
        assert_eq!(test_goju_money_number(5230),7);
    }
    #[test]
    fn t_5230_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5230),7);
    }
    #[test]
    fn t_5230_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5230),5);
    }
    #[test]
    fn t_5230_sen_test(){
        assert_eq!(test_sen_money_number(5230),19);
    }
    #[test]
    fn t_5230_gosen_test(){
        assert_eq!(test_gosen_money_number(5230),-5);
    }
    #[test]
    fn t_5230_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5230),-5);
    }
    // 5240
    #[test]
    fn t_5240_ju_test(){
        assert_eq!(test_ju_money_number(5240),5);
    }
    #[test]
    fn t_5240_goju_test(){
        assert_eq!(test_goju_money_number(5240),7);
    }
    #[test]
    fn t_5240_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5240),7);
    }
    #[test]
    fn t_5240_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5240),5);
    }
    #[test]
    fn t_5240_sen_test(){
        assert_eq!(test_sen_money_number(5240),19);
    }
    #[test]
    fn t_5240_gosen_test(){
        assert_eq!(test_gosen_money_number(5240),-5);
    }
    #[test]
    fn t_5240_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5240),-5);
    }
    // 5250
    #[test]
    fn t_5250_ju_test(){
        assert_eq!(test_ju_money_number(5250),0);
    }
    #[test]
    fn t_5250_goju_test(){
        assert_eq!(test_goju_money_number(5250),6);
    }
    #[test]
    fn t_5250_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5250),2);
    }
    #[test]
    fn t_5250_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5250),2);
    }
    #[test]
    fn t_5250_sen_test(){
        assert_eq!(test_sen_money_number(5250),6);
    }
    #[test]
    fn t_5250_gosen_test(){
        assert_eq!(test_gosen_money_number(5250),-8);
    }
    #[test]
    fn t_5250_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5250),-2);
    }
    // 5260
    #[test]
    fn t_5260_ju_test(){
        assert_eq!(test_ju_money_number(5260),30);
    }
    #[test]
    fn t_5260_goju_test(){
        assert_eq!(test_goju_money_number(5260),-2);
    }
    #[test]
    fn t_5260_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5260),2);
    }
    #[test]
    fn t_5260_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5260),2);
    }
    #[test]
    fn t_5260_sen_test(){
        assert_eq!(test_sen_money_number(5260),6);
    }
    #[test]
    fn t_5260_gosen_test(){
        assert_eq!(test_gosen_money_number(5260),-8);
    }
    #[test]
    fn t_5260_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5260),-2);
    }
    // 5270
    #[test]
    fn t_5270_ju_test(){
        assert_eq!(test_ju_money_number(5270),20);
    }
    #[test]
    fn t_5270_goju_test(){
        assert_eq!(test_goju_money_number(5270),-2);
    }
    #[test]
    fn t_5270_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5270),2);
    }
    #[test]
    fn t_5270_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5270),2);
    }
    #[test]
    fn t_5270_sen_test(){
        assert_eq!(test_sen_money_number(5270),6);
    }
    #[test]
    fn t_5270_gosen_test(){
        assert_eq!(test_gosen_money_number(5270),-8);
    }
    #[test]
    fn t_5270_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5270),-2);
    }
    // 5280
    #[test]
    fn t_5280_ju_test(){
        assert_eq!(test_ju_money_number(5280),10);
    }
    #[test]
    fn t_5280_goju_test(){
        assert_eq!(test_goju_money_number(5280),-2);
    }
    #[test]
    fn t_5280_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5280),2);
    }
    #[test]
    fn t_5280_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5280),2);
    }
    #[test]
    fn t_5280_sen_test(){
        assert_eq!(test_sen_money_number(5280),6);
    }
    #[test]
    fn t_5280_gosen_test(){
        assert_eq!(test_gosen_money_number(5280),-8);
    }
    #[test]
    fn t_5280_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5280),-2);
    }
    // 5290
    #[test]
    fn t_5290_ju_test(){
        assert_eq!(test_ju_money_number(5290),0);
    }
    #[test]
    fn t_5290_goju_test(){
        assert_eq!(test_goju_money_number(5290),-2);
    }
    #[test]
    fn t_5290_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5290),2);
    }
    #[test]
    fn t_5290_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5290),2);
    }
    #[test]
    fn t_5290_sen_test(){
        assert_eq!(test_sen_money_number(5290),6);
    }
    #[test]
    fn t_5290_gosen_test(){
        assert_eq!(test_gosen_money_number(5290),-8);
    }
    #[test]
    fn t_5290_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5290),-2);
    }
    // 5300
    #[test]
    fn t_5300_ju_test(){
        assert_eq!(test_ju_money_number(5300),0);
    }
    #[test]
    fn t_5300_goju_test(){
        assert_eq!(test_goju_money_number(5300),0);
    }
    #[test]
    fn t_5300_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5300),10);
    }
    #[test]
    fn t_5300_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5300),4);
    }
    #[test]
    fn t_5300_sen_test(){
        assert_eq!(test_sen_money_number(5300),14);
    }
    #[test]
    fn t_5300_gosen_test(){
        assert_eq!(test_gosen_money_number(5300),-6);
    }
    #[test]
    fn t_5300_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5300),-4);
    }
    // 5310
    #[test]
    fn t_5310_ju_test(){
        assert_eq!(test_ju_money_number(5310),35);
    }
    #[test]
    fn t_5310_goju_test(){
        assert_eq!(test_goju_money_number(5310),7);
    }
    #[test]
    fn t_5310_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5310),-3);
    }
    #[test]
    fn t_5310_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5310),5);
    }
    #[test]
    fn t_5310_sen_test(){
        assert_eq!(test_sen_money_number(5310),19);
    }
    #[test]
    fn t_5310_gosen_test(){
        assert_eq!(test_gosen_money_number(5310),-5);
    }
    #[test]
    fn t_5310_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5310),-5);
    }
    // 5320
    #[test]
    fn t_5320_ju_test(){
        assert_eq!(test_ju_money_number(5320),25);
    }
    #[test]
    fn t_5320_goju_test(){
        assert_eq!(test_goju_money_number(5320),7);
    }
    #[test]
    fn t_5320_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5320),-3);
    }
    #[test]
    fn t_5320_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5320),5);
    }
    #[test]
    fn t_5320_sen_test(){
        assert_eq!(test_sen_money_number(5320),19);
    }
    #[test]
    fn t_5320_gosen_test(){
        assert_eq!(test_gosen_money_number(5320),-5);
    }
    #[test]
    fn t_5320_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5320),-5);
    }
    // 5330
    #[test]
    fn t_5330_ju_test(){
        assert_eq!(test_ju_money_number(5330),15);
    }
    #[test]
    fn t_5330_goju_test(){
        assert_eq!(test_goju_money_number(5330),7);
    }
    #[test]
    fn t_5330_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5330),-3);
    }
    #[test]
    fn t_5330_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5330),5);
    }
    #[test]
    fn t_5330_sen_test(){
        assert_eq!(test_sen_money_number(5330),19);
    }
    #[test]
    fn t_5330_gosen_test(){
        assert_eq!(test_gosen_money_number(5330),-5);
    }
    #[test]
    fn t_5330_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5330),-5);
    }
    // 5340
    #[test]
    fn t_5340_ju_test(){
        assert_eq!(test_ju_money_number(5340),5);
    }
    #[test]
    fn t_5340_goju_test(){
        assert_eq!(test_goju_money_number(5340),7);
    }
    #[test]
    fn t_5340_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5340),-3);
    }
    #[test]
    fn t_5340_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5340),5);
    }
    #[test]
    fn t_5340_sen_test(){
        assert_eq!(test_sen_money_number(5340),19);
    }
    #[test]
    fn t_5340_gosen_test(){
        assert_eq!(test_gosen_money_number(5340),-5);
    }
    #[test]
    fn t_5340_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5340),-5);
    }
    // 5350
    #[test]
    fn t_5350_ju_test(){
        assert_eq!(test_ju_money_number(5350),0);
    }
    #[test]
    fn t_5350_goju_test(){
        assert_eq!(test_goju_money_number(5350),6);
    }
    #[test]
    fn t_5350_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5350),-8);
    }
    #[test]
    fn t_5350_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5350),2);
    }
    #[test]
    fn t_5350_sen_test(){
        assert_eq!(test_sen_money_number(5350),6);
    }
    #[test]
    fn t_5350_gosen_test(){
        assert_eq!(test_gosen_money_number(5350),-8);
    }
    #[test]
    fn t_5350_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5350),-2);
    }
    // 5360
    #[test]
    fn t_5360_ju_test(){
        assert_eq!(test_ju_money_number(5360),30);
    }
    #[test]
    fn t_5360_goju_test(){
        assert_eq!(test_goju_money_number(5360),-2);
    }
    #[test]
    fn t_5360_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5360),-8);
    }
    #[test]
    fn t_5360_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5360),2);
    }
    #[test]
    fn t_5360_sen_test(){
        assert_eq!(test_sen_money_number(5360),6);
    }
    #[test]
    fn t_5360_gosen_test(){
        assert_eq!(test_gosen_money_number(5360),-8);
    }
    #[test]
    fn t_5360_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5360),-2);
    }
    // 5370
    #[test]
    fn t_5370_ju_test(){
        assert_eq!(test_ju_money_number(5370),20);
    }
    #[test]
    fn t_5370_goju_test(){
        assert_eq!(test_goju_money_number(5370),-2);
    }
    #[test]
    fn t_5370_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5370),-8);
    }
    #[test]
    fn t_5370_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5370),2);
    }
    #[test]
    fn t_5370_sen_test(){
        assert_eq!(test_sen_money_number(5370),6);
    }
    #[test]
    fn t_5370_gosen_test(){
        assert_eq!(test_gosen_money_number(5370),-8);
    }
    #[test]
    fn t_5370_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5370),-2);
    }
    // 5380
    #[test]
    fn t_5380_ju_test(){
        assert_eq!(test_ju_money_number(5380),10);
    }
    #[test]
    fn t_5380_goju_test(){
        assert_eq!(test_goju_money_number(5380),-2);
    }
    #[test]
    fn t_5380_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5380),-8);
    }
    #[test]
    fn t_5380_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5380),2);
    }
    #[test]
    fn t_5380_sen_test(){
        assert_eq!(test_sen_money_number(5380),6);
    }
    #[test]
    fn t_5380_gosen_test(){
        assert_eq!(test_gosen_money_number(5380),-8);
    }
    #[test]
    fn t_5380_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5380),-2);
    }
    // 5390
    #[test]
    fn t_5390_ju_test(){
        assert_eq!(test_ju_money_number(5390),0);
    }
    #[test]
    fn t_5390_goju_test(){
        assert_eq!(test_goju_money_number(5390),-2);
    }
    #[test]
    fn t_5390_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5390),-8);
    }
    #[test]
    fn t_5390_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5390),2);
    }
    #[test]
    fn t_5390_sen_test(){
        assert_eq!(test_sen_money_number(5390),6);
    }
    #[test]
    fn t_5390_gosen_test(){
        assert_eq!(test_gosen_money_number(5390),-8);
    }
    #[test]
    fn t_5390_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5390),-2);
    }
    // 5400
    #[test]
    fn t_5400_ju_test(){
        assert_eq!(test_ju_money_number(5400),0);
    }
    #[test]
    fn t_5400_goju_test(){
        assert_eq!(test_goju_money_number(5400),0);
    }
    #[test]
    fn t_5400_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5400),0);
    }
    #[test]
    fn t_5400_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5400),4);
    }
    #[test]
    fn t_5400_sen_test(){
        assert_eq!(test_sen_money_number(5400),14);
    }
    #[test]
    fn t_5400_gosen_test(){
        assert_eq!(test_gosen_money_number(5400),-6);
    }
    #[test]
    fn t_5400_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5400),-4);
    }
    // 5410
    #[test]
    fn t_5410_ju_test(){
        assert_eq!(test_ju_money_number(5410),30);
    }
    #[test]
    fn t_5410_goju_test(){
        assert_eq!(test_goju_money_number(5410),4);
    }
    #[test]
    fn t_5410_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5410),-16);
    }
    #[test]
    fn t_5410_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5410),2);
    }
    #[test]
    fn t_5410_sen_test(){
        assert_eq!(test_sen_money_number(5410),6);
    }
    #[test]
    fn t_5410_gosen_test(){
        assert_eq!(test_gosen_money_number(5410),-8);
    }
    #[test]
    fn t_5410_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5410),-2);
    }
    // 5420
    #[test]
    fn t_5420_ju_test(){
        assert_eq!(test_ju_money_number(5420),20);
    }
    #[test]
    fn t_5420_goju_test(){
        assert_eq!(test_goju_money_number(5420),4);
    }
    #[test]
    fn t_5420_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5420),-16);
    }
    #[test]
    fn t_5420_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5420),2);
    }
    #[test]
    fn t_5420_sen_test(){
        assert_eq!(test_sen_money_number(5420),6);
    }
    #[test]
    fn t_5420_gosen_test(){
        assert_eq!(test_gosen_money_number(5420),-8);
    }
    #[test]
    fn t_5420_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5420),-2);
    }
    // 5430
    #[test]
    fn t_5430_ju_test(){
        assert_eq!(test_ju_money_number(5430),10);
    }
    #[test]
    fn t_5430_goju_test(){
        assert_eq!(test_goju_money_number(5430),4);
    }
    #[test]
    fn t_5430_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5430),-16);
    }
    #[test]
    fn t_5430_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5430),2);
    }
    #[test]
    fn t_5430_sen_test(){
        assert_eq!(test_sen_money_number(5430),6);
    }
    #[test]
    fn t_5430_gosen_test(){
        assert_eq!(test_gosen_money_number(5430),-8);
    }
    #[test]
    fn t_5430_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5430),-2);
    }
    // 5440
    #[test]
    fn t_5440_ju_test(){
        assert_eq!(test_ju_money_number(5440),0);
    }
    #[test]
    fn t_5440_goju_test(){
        assert_eq!(test_goju_money_number(5440),4);
    }
    #[test]
    fn t_5440_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5440),-16);
    }
    #[test]
    fn t_5440_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5440),2);
    }
    #[test]
    fn t_5440_sen_test(){
        assert_eq!(test_sen_money_number(5440),6);
    }
    #[test]
    fn t_5440_gosen_test(){
        assert_eq!(test_gosen_money_number(5440),-8);
    }
    #[test]
    fn t_5440_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5440),-2);
    }
    // 5450
    #[test]
    fn t_5450_ju_test(){
        assert_eq!(test_ju_money_number(5450),0);
    }
    #[test]
    fn t_5450_goju_test(){
        assert_eq!(test_goju_money_number(5450),6);
    }
    #[test]
    fn t_5450_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5450),-8);
    }
    #[test]
    fn t_5450_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5450),4);
    }
    #[test]
    fn t_5450_sen_test(){
        assert_eq!(test_sen_money_number(5450),14);
    }
    #[test]
    fn t_5450_gosen_test(){
        assert_eq!(test_gosen_money_number(5450),-6);
    }
    #[test]
    fn t_5450_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5450),-4);
    }
    // 5460
    #[test]
    fn t_5460_ju_test(){
        assert_eq!(test_ju_money_number(5460),30);
    }
    #[test]
    fn t_5460_goju_test(){
        assert_eq!(test_goju_money_number(5460),-2);
    }
    #[test]
    fn t_5460_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5460),-8);
    }
    #[test]
    fn t_5460_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5460),4);
    }
    #[test]
    fn t_5460_sen_test(){
        assert_eq!(test_sen_money_number(5460),14);
    }
    #[test]
    fn t_5460_gosen_test(){
        assert_eq!(test_gosen_money_number(5460),-6);
    }
    #[test]
    fn t_5460_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5460),-4);
    }
    // 5470
    #[test]
    fn t_5470_ju_test(){
        assert_eq!(test_ju_money_number(5470),20);
    }
    #[test]
    fn t_5470_goju_test(){
        assert_eq!(test_goju_money_number(5470),-2);
    }
    #[test]
    fn t_5470_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5470),-8);
    }
    #[test]
    fn t_5470_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5470),4);
    }
    #[test]
    fn t_5470_sen_test(){
        assert_eq!(test_sen_money_number(5470),14);
    }
    #[test]
    fn t_5470_gosen_test(){
        assert_eq!(test_gosen_money_number(5470),-6);
    }
    #[test]
    fn t_5470_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5470),-4);
    }
    // 5480
    #[test]
    fn t_5480_ju_test(){
        assert_eq!(test_ju_money_number(5480),10);
    }
    #[test]
    fn t_5480_goju_test(){
        assert_eq!(test_goju_money_number(5480),-2);
    }
    #[test]
    fn t_5480_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5480),-8);
    }
    #[test]
    fn t_5480_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5480),4);
    }
    #[test]
    fn t_5480_sen_test(){
        assert_eq!(test_sen_money_number(5480),14);
    }
    #[test]
    fn t_5480_gosen_test(){
        assert_eq!(test_gosen_money_number(5480),-6);
    }
    #[test]
    fn t_5480_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5480),-4);
    }
    // 5490
    #[test]
    fn t_5490_ju_test(){
        assert_eq!(test_ju_money_number(5490),0);
    }
    #[test]
    fn t_5490_goju_test(){
        assert_eq!(test_goju_money_number(5490),-2);
    }
    #[test]
    fn t_5490_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5490),-8);
    }
    #[test]
    fn t_5490_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5490),4);
    }
    #[test]
    fn t_5490_sen_test(){
        assert_eq!(test_sen_money_number(5490),14);
    }
    #[test]
    fn t_5490_gosen_test(){
        assert_eq!(test_gosen_money_number(5490),-6);
    }
    #[test]
    fn t_5490_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5490),-4);
    }
    // 5500
    #[test]
    fn t_5500_ju_test(){
        assert_eq!(test_ju_money_number(5500),0);
    }
    #[test]
    fn t_5500_goju_test(){
        assert_eq!(test_goju_money_number(5500),0);
    }
    #[test]
    fn t_5500_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5500),0);
    }
    #[test]
    fn t_5500_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5500),4);
    }
    #[test]
    fn t_5500_sen_test(){
        assert_eq!(test_sen_money_number(5500),13);
    }
    #[test]
    fn t_5500_gosen_test(){
        assert_eq!(test_gosen_money_number(5500),-6);
    }
    #[test]
    fn t_5500_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5500),-4);
    }
    // 5510
    #[test]
    fn t_5510_ju_test(){
        assert_eq!(test_ju_money_number(5510),30);
    }
    #[test]
    fn t_5510_goju_test(){
        assert_eq!(test_goju_money_number(5510),4);
    }
    #[test]
    fn t_5510_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5510),14);
    }
    #[test]
    fn t_5510_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5510),-6);
    }
    #[test]
    fn t_5510_sen_test(){
        assert_eq!(test_sen_money_number(5510),6);
    }
    #[test]
    fn t_5510_gosen_test(){
        assert_eq!(test_gosen_money_number(5510),-8);
    }
    #[test]
    fn t_5510_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5510),-2);
    }
    // 5520
    #[test]
    fn t_5520_ju_test(){
        assert_eq!(test_ju_money_number(5520),20);
    }
    #[test]
    fn t_5520_goju_test(){
        assert_eq!(test_goju_money_number(5520),4);
    }
    #[test]
    fn t_5520_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5520),14);
    }
    #[test]
    fn t_5520_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5520),-6);
    }
    #[test]
    fn t_5520_sen_test(){
        assert_eq!(test_sen_money_number(5520),6);
    }
    #[test]
    fn t_5520_gosen_test(){
        assert_eq!(test_gosen_money_number(5520),-8);
    }
    #[test]
    fn t_5520_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5520),-2);
    }
    // 5530
    #[test]
    fn t_5530_ju_test(){
        assert_eq!(test_ju_money_number(5530),10);
    }
    #[test]
    fn t_5530_goju_test(){
        assert_eq!(test_goju_money_number(5530),4);
    }
    #[test]
    fn t_5530_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5530),14);
    }
    #[test]
    fn t_5530_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5530),-6);
    }
    #[test]
    fn t_5530_sen_test(){
        assert_eq!(test_sen_money_number(5530),6);
    }
    #[test]
    fn t_5530_gosen_test(){
        assert_eq!(test_gosen_money_number(5530),-8);
    }
    #[test]
    fn t_5530_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5530),-2);
    }
    // 5540
    #[test]
    fn t_5540_ju_test(){
        assert_eq!(test_ju_money_number(5540),0);
    }
    #[test]
    fn t_5540_goju_test(){
        assert_eq!(test_goju_money_number(5540),4);
    }
    #[test]
    fn t_5540_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5540),14);
    }
    #[test]
    fn t_5540_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5540),-6);
    }
    #[test]
    fn t_5540_sen_test(){
        assert_eq!(test_sen_money_number(5540),6);
    }
    #[test]
    fn t_5540_gosen_test(){
        assert_eq!(test_gosen_money_number(5540),-8);
    }
    #[test]
    fn t_5540_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5540),-2);
    }
    // 5550
    #[test]
    fn t_5550_ju_test(){
        assert_eq!(test_ju_money_number(5550),0);
    }
    #[test]
    fn t_5550_goju_test(){
        assert_eq!(test_goju_money_number(5550),6);
    }
    #[test]
    fn t_5550_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5550),22);
    }
    #[test]
    fn t_5550_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5550),-4);
    }
    #[test]
    fn t_5550_sen_test(){
        assert_eq!(test_sen_money_number(5550),14);
    }
    #[test]
    fn t_5550_gosen_test(){
        assert_eq!(test_gosen_money_number(5550),-6);
    }
    #[test]
    fn t_5550_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5550),-4);
    }
    // 5560
    #[test]
    fn t_5560_ju_test(){
        assert_eq!(test_ju_money_number(5560),30);
    }
    #[test]
    fn t_5560_goju_test(){
        assert_eq!(test_goju_money_number(5560),-2);
    }
    #[test]
    fn t_5560_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5560),22);
    }
    #[test]
    fn t_5560_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5560),-4);
    }
    #[test]
    fn t_5560_sen_test(){
        assert_eq!(test_sen_money_number(5560),14);
    }
    #[test]
    fn t_5560_gosen_test(){
        assert_eq!(test_gosen_money_number(5560),-6);
    }
    #[test]
    fn t_5560_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5560),-4);
    }
    // 5570
    #[test]
    fn t_5570_ju_test(){
        assert_eq!(test_ju_money_number(5570),20);
    }
    #[test]
    fn t_5570_goju_test(){
        assert_eq!(test_goju_money_number(5570),-2);
    }
    #[test]
    fn t_5570_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5570),22);
    }
    #[test]
    fn t_5570_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5570),-4);
    }
    #[test]
    fn t_5570_sen_test(){
        assert_eq!(test_sen_money_number(5570),14);
    }
    #[test]
    fn t_5570_gosen_test(){
        assert_eq!(test_gosen_money_number(5570),-6);
    }
    #[test]
    fn t_5570_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5570),-4);
    }
    // 5580
    #[test]
    fn t_5580_ju_test(){
        assert_eq!(test_ju_money_number(5580),10);
    }
    #[test]
    fn t_5580_goju_test(){
        assert_eq!(test_goju_money_number(5580),-2);
    }
    #[test]
    fn t_5580_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5580),22);
    }
    #[test]
    fn t_5580_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5580),-4);
    }
    #[test]
    fn t_5580_sen_test(){
        assert_eq!(test_sen_money_number(5580),14);
    }
    #[test]
    fn t_5580_gosen_test(){
        assert_eq!(test_gosen_money_number(5580),-6);
    }
    #[test]
    fn t_5580_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5580),-4);
    }
    // 5590
    #[test]
    fn t_5590_ju_test(){
        assert_eq!(test_ju_money_number(5590),0);
    }
    #[test]
    fn t_5590_goju_test(){
        assert_eq!(test_goju_money_number(5590),-2);
    }
    #[test]
    fn t_5590_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5590),22);
    }
    #[test]
    fn t_5590_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5590),-4);
    }
    #[test]
    fn t_5590_sen_test(){
        assert_eq!(test_sen_money_number(5590),14);
    }
    #[test]
    fn t_5590_gosen_test(){
        assert_eq!(test_gosen_money_number(5590),-6);
    }
    #[test]
    fn t_5590_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5590),-4);
    }
    // 5600
    #[test]
    fn t_5600_ju_test(){
        assert_eq!(test_ju_money_number(5600),0);
    }
    #[test]
    fn t_5600_goju_test(){
        assert_eq!(test_goju_money_number(5600),0);
    }
    #[test]
    fn t_5600_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5600),25);
    }
    #[test]
    fn t_5600_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5600),-3);
    }
    #[test]
    fn t_5600_sen_test(){
        assert_eq!(test_sen_money_number(5600),13);
    }
    #[test]
    fn t_5600_gosen_test(){
        assert_eq!(test_gosen_money_number(5600),-6);
    }
    #[test]
    fn t_5600_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5600),-4);
    }
    // 5610
    #[test]
    fn t_5610_ju_test(){
        assert_eq!(test_ju_money_number(5610),30);
    }
    #[test]
    fn t_5610_goju_test(){
        assert_eq!(test_goju_money_number(5610),4);
    }
    #[test]
    fn t_5610_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5610),4);
    }
    #[test]
    fn t_5610_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5610),-6);
    }
    #[test]
    fn t_5610_sen_test(){
        assert_eq!(test_sen_money_number(5610),6);
    }
    #[test]
    fn t_5610_gosen_test(){
        assert_eq!(test_gosen_money_number(5610),-8);
    }
    #[test]
    fn t_5610_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5610),-2);
    }
    // 5620
    #[test]
    fn t_5620_ju_test(){
        assert_eq!(test_ju_money_number(5620),20);
    }
    #[test]
    fn t_5620_goju_test(){
        assert_eq!(test_goju_money_number(5620),4);
    }
    #[test]
    fn t_5620_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5620),4);
    }
    #[test]
    fn t_5620_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5620),-6);
    }
    #[test]
    fn t_5620_sen_test(){
        assert_eq!(test_sen_money_number(5620),6);
    }
    #[test]
    fn t_5620_gosen_test(){
        assert_eq!(test_gosen_money_number(5620),-8);
    }
    #[test]
    fn t_5620_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5620),-2);
    }
    // 5630
    #[test]
    fn t_5630_ju_test(){
        assert_eq!(test_ju_money_number(5630),10);
    }
    #[test]
    fn t_5630_goju_test(){
        assert_eq!(test_goju_money_number(5630),4);
    }
    #[test]
    fn t_5630_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5630),4);
    }
    #[test]
    fn t_5630_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5630),-6);
    }
    #[test]
    fn t_5630_sen_test(){
        assert_eq!(test_sen_money_number(5630),6);
    }
    #[test]
    fn t_5630_gosen_test(){
        assert_eq!(test_gosen_money_number(5630),-8);
    }
    #[test]
    fn t_5630_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5630),-2);
    }
    // 5640
    #[test]
    fn t_5640_ju_test(){
        assert_eq!(test_ju_money_number(5640),0);
    }
    #[test]
    fn t_5640_goju_test(){
        assert_eq!(test_goju_money_number(5640),4);
    }
    #[test]
    fn t_5640_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5640),4);
    }
    #[test]
    fn t_5640_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5640),-6);
    }
    #[test]
    fn t_5640_sen_test(){
        assert_eq!(test_sen_money_number(5640),6);
    }
    #[test]
    fn t_5640_gosen_test(){
        assert_eq!(test_gosen_money_number(5640),-8);
    }
    #[test]
    fn t_5640_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5640),-2);
    }
    // 5650
    #[test]
    fn t_5650_ju_test(){
        assert_eq!(test_ju_money_number(5650),0);
    }
    #[test]
    fn t_5650_goju_test(){
        assert_eq!(test_goju_money_number(5650),6);
    }
    #[test]
    fn t_5650_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5650),12);
    }
    #[test]
    fn t_5650_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5650),-4);
    }
    #[test]
    fn t_5650_sen_test(){
        assert_eq!(test_sen_money_number(5650),14);
    }
    #[test]
    fn t_5650_gosen_test(){
        assert_eq!(test_gosen_money_number(5650),-6);
    }
    #[test]
    fn t_5650_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5650),-4);
    }
    // 5660
    #[test]
    fn t_5660_ju_test(){
        assert_eq!(test_ju_money_number(5660),30);
    }
    #[test]
    fn t_5660_goju_test(){
        assert_eq!(test_goju_money_number(5660),-2);
    }
    #[test]
    fn t_5660_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5660),12);
    }
    #[test]
    fn t_5660_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5660),-4);
    }
    #[test]
    fn t_5660_sen_test(){
        assert_eq!(test_sen_money_number(5660),14);
    }
    #[test]
    fn t_5660_gosen_test(){
        assert_eq!(test_gosen_money_number(5660),-6);
    }
    #[test]
    fn t_5660_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5660),-4);
    }
    // 5670
    #[test]
    fn t_5670_ju_test(){
        assert_eq!(test_ju_money_number(5670),20);
    }
    #[test]
    fn t_5670_goju_test(){
        assert_eq!(test_goju_money_number(5670),-2);
    }
    #[test]
    fn t_5670_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5670),12);
    }
    #[test]
    fn t_5670_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5670),-4);
    }
    #[test]
    fn t_5670_sen_test(){
        assert_eq!(test_sen_money_number(5670),14);
    }
    #[test]
    fn t_5670_gosen_test(){
        assert_eq!(test_gosen_money_number(5670),-6);
    }
    #[test]
    fn t_5670_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5670),-4);
    }
    // 5680
    #[test]
    fn t_5680_ju_test(){
        assert_eq!(test_ju_money_number(5680),10);
    }
    #[test]
    fn t_5680_goju_test(){
        assert_eq!(test_goju_money_number(5680),-2);
    }
    #[test]
    fn t_5680_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5680),12);
    }
    #[test]
    fn t_5680_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5680),-4);
    }
    #[test]
    fn t_5680_sen_test(){
        assert_eq!(test_sen_money_number(5680),14);
    }
    #[test]
    fn t_5680_gosen_test(){
        assert_eq!(test_gosen_money_number(5680),-6);
    }
    #[test]
    fn t_5680_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5680),-4);
    }
    // 5690
    #[test]
    fn t_5690_ju_test(){
        assert_eq!(test_ju_money_number(5690),0);
    }
    #[test]
    fn t_5690_goju_test(){
        assert_eq!(test_goju_money_number(5690),-2);
    }
    #[test]
    fn t_5690_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5690),12);
    }
    #[test]
    fn t_5690_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5690),-4);
    }
    #[test]
    fn t_5690_sen_test(){
        assert_eq!(test_sen_money_number(5690),14);
    }
    #[test]
    fn t_5690_gosen_test(){
        assert_eq!(test_gosen_money_number(5690),-6);
    }
    #[test]
    fn t_5690_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5690),-4);
    }
    // 5700
    #[test]
    fn t_5700_ju_test(){
        assert_eq!(test_ju_money_number(5700),0);
    }
    #[test]
    fn t_5700_goju_test(){
        assert_eq!(test_goju_money_number(5700),0);
    }
    #[test]
    fn t_5700_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5700),15);
    }
    #[test]
    fn t_5700_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5700),-3);
    }
    #[test]
    fn t_5700_sen_test(){
        assert_eq!(test_sen_money_number(5700),13);
    }
    #[test]
    fn t_5700_gosen_test(){
        assert_eq!(test_gosen_money_number(5700),-6);
    }
    #[test]
    fn t_5700_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5700),-4);
    }
    // 5710
    #[test]
    fn t_5710_ju_test(){
        assert_eq!(test_ju_money_number(5710),30);
    }
    #[test]
    fn t_5710_goju_test(){
        assert_eq!(test_goju_money_number(5710),4);
    }
    #[test]
    fn t_5710_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5710),-6);
    }
    #[test]
    fn t_5710_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5710),-6);
    }
    #[test]
    fn t_5710_sen_test(){
        assert_eq!(test_sen_money_number(5710),6);
    }
    #[test]
    fn t_5710_gosen_test(){
        assert_eq!(test_gosen_money_number(5710),-8);
    }
    #[test]
    fn t_5710_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5710),-2);
    }
    // 5720
    #[test]
    fn t_5720_ju_test(){
        assert_eq!(test_ju_money_number(5720),20);
    }
    #[test]
    fn t_5720_goju_test(){
        assert_eq!(test_goju_money_number(5720),4);
    }
    #[test]
    fn t_5720_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5720),-6);
    }
    #[test]
    fn t_5720_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5720),-6);
    }
    #[test]
    fn t_5720_sen_test(){
        assert_eq!(test_sen_money_number(5720),6);
    }
    #[test]
    fn t_5720_gosen_test(){
        assert_eq!(test_gosen_money_number(5720),-8);
    }
    #[test]
    fn t_5720_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5720),-2);
    }
    // 5730
    #[test]
    fn t_5730_ju_test(){
        assert_eq!(test_ju_money_number(5730),10);
    }
    #[test]
    fn t_5730_goju_test(){
        assert_eq!(test_goju_money_number(5730),4);
    }
    #[test]
    fn t_5730_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5730),-6);
    }
    #[test]
    fn t_5730_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5730),-6);
    }
    #[test]
    fn t_5730_sen_test(){
        assert_eq!(test_sen_money_number(5730),6);
    }
    #[test]
    fn t_5730_gosen_test(){
        assert_eq!(test_gosen_money_number(5730),-8);
    }
    #[test]
    fn t_5730_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5730),-2);
    }
    // 5740
    #[test]
    fn t_5740_ju_test(){
        assert_eq!(test_ju_money_number(5740),0);
    }
    #[test]
    fn t_5740_goju_test(){
        assert_eq!(test_goju_money_number(5740),4);
    }
    #[test]
    fn t_5740_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5740),-6);
    }
    #[test]
    fn t_5740_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5740),-6);
    }
    #[test]
    fn t_5740_sen_test(){
        assert_eq!(test_sen_money_number(5740),6);
    }
    #[test]
    fn t_5740_gosen_test(){
        assert_eq!(test_gosen_money_number(5740),-8);
    }
    #[test]
    fn t_5740_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5740),-2);
    }
    // 5750
    #[test]
    fn t_5750_ju_test(){
        assert_eq!(test_ju_money_number(5750),0);
    }
    #[test]
    fn t_5750_goju_test(){
        assert_eq!(test_goju_money_number(5750),6);
    }
    #[test]
    fn t_5750_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5750),2);
    }
    #[test]
    fn t_5750_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5750),-4);
    }
    #[test]
    fn t_5750_sen_test(){
        assert_eq!(test_sen_money_number(5750),14);
    }
    #[test]
    fn t_5750_gosen_test(){
        assert_eq!(test_gosen_money_number(5750),-6);
    }
    #[test]
    fn t_5750_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5750),-4);
    }
    // 5760
    #[test]
    fn t_5760_ju_test(){
        assert_eq!(test_ju_money_number(5760),30);
    }
    #[test]
    fn t_5760_goju_test(){
        assert_eq!(test_goju_money_number(5760),-2);
    }
    #[test]
    fn t_5760_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5760),2);
    }
    #[test]
    fn t_5760_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5760),-4);
    }
    #[test]
    fn t_5760_sen_test(){
        assert_eq!(test_sen_money_number(5760),14);
    }
    #[test]
    fn t_5760_gosen_test(){
        assert_eq!(test_gosen_money_number(5760),-6);
    }
    #[test]
    fn t_5760_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5760),-4);
    }
    // 5770
    #[test]
    fn t_5770_ju_test(){
        assert_eq!(test_ju_money_number(5770),20);
    }
    #[test]
    fn t_5770_goju_test(){
        assert_eq!(test_goju_money_number(5770),-2);
    }
    #[test]
    fn t_5770_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5770),2);
    }
    #[test]
    fn t_5770_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5770),-4);
    }
    #[test]
    fn t_5770_sen_test(){
        assert_eq!(test_sen_money_number(5770),14);
    }
    #[test]
    fn t_5770_gosen_test(){
        assert_eq!(test_gosen_money_number(5770),-6);
    }
    #[test]
    fn t_5770_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5770),-4);
    }
    // 5780
    #[test]
    fn t_5780_ju_test(){
        assert_eq!(test_ju_money_number(5780),10);
    }
    #[test]
    fn t_5780_goju_test(){
        assert_eq!(test_goju_money_number(5780),-2);
    }
    #[test]
    fn t_5780_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5780),2);
    }
    #[test]
    fn t_5780_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5780),-4);
    }
    #[test]
    fn t_5780_sen_test(){
        assert_eq!(test_sen_money_number(5780),14);
    }
    #[test]
    fn t_5780_gosen_test(){
        assert_eq!(test_gosen_money_number(5780),-6);
    }
    #[test]
    fn t_5780_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5780),-4);
    }
    // 5790
    #[test]
    fn t_5790_ju_test(){
        assert_eq!(test_ju_money_number(5790),0);
    }
    #[test]
    fn t_5790_goju_test(){
        assert_eq!(test_goju_money_number(5790),-2);
    }
    #[test]
    fn t_5790_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5790),2);
    }
    #[test]
    fn t_5790_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5790),-4);
    }
    #[test]
    fn t_5790_sen_test(){
        assert_eq!(test_sen_money_number(5790),14);
    }
    #[test]
    fn t_5790_gosen_test(){
        assert_eq!(test_gosen_money_number(5790),-6);
    }
    #[test]
    fn t_5790_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5790),-4);
    }
    // 5800
    #[test]
    fn t_5800_ju_test(){
        assert_eq!(test_ju_money_number(5800),0);
    }
    #[test]
    fn t_5800_goju_test(){
        assert_eq!(test_goju_money_number(5800),0);
    }
    #[test]
    fn t_5800_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5800),5);
    }
    #[test]
    fn t_5800_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5800),-3);
    }
    #[test]
    fn t_5800_sen_test(){
        assert_eq!(test_sen_money_number(5800),13);
    }
    #[test]
    fn t_5800_gosen_test(){
        assert_eq!(test_gosen_money_number(5800),-6);
    }
    #[test]
    fn t_5800_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5800),-4);
    }
    // 5810
    #[test]
    fn t_5810_ju_test(){
        assert_eq!(test_ju_money_number(5810),30);
    }
    #[test]
    fn t_5810_goju_test(){
        assert_eq!(test_goju_money_number(5810),4);
    }
    #[test]
    fn t_5810_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5810),-16);
    }
    #[test]
    fn t_5810_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5810),-6);
    }
    #[test]
    fn t_5810_sen_test(){
        assert_eq!(test_sen_money_number(5810),6);
    }
    #[test]
    fn t_5810_gosen_test(){
        assert_eq!(test_gosen_money_number(5810),-8);
    }
    #[test]
    fn t_5810_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5810),-2);
    }
    // 5820
    #[test]
    fn t_5820_ju_test(){
        assert_eq!(test_ju_money_number(5820),20);
    }
    #[test]
    fn t_5820_goju_test(){
        assert_eq!(test_goju_money_number(5820),4);
    }
    #[test]
    fn t_5820_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5820),-16);
    }
    #[test]
    fn t_5820_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5820),-6);
    }
    #[test]
    fn t_5820_sen_test(){
        assert_eq!(test_sen_money_number(5820),6);
    }
    #[test]
    fn t_5820_gosen_test(){
        assert_eq!(test_gosen_money_number(5820),-8);
    }
    #[test]
    fn t_5820_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5820),-2);
    }
    // 5830
    #[test]
    fn t_5830_ju_test(){
        assert_eq!(test_ju_money_number(5830),10);
    }
    #[test]
    fn t_5830_goju_test(){
        assert_eq!(test_goju_money_number(5830),4);
    }
    #[test]
    fn t_5830_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5830),-16);
    }
    #[test]
    fn t_5830_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5830),-6);
    }
    #[test]
    fn t_5830_sen_test(){
        assert_eq!(test_sen_money_number(5830),6);
    }
    #[test]
    fn t_5830_gosen_test(){
        assert_eq!(test_gosen_money_number(5830),-8);
    }
    #[test]
    fn t_5830_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5830),-2);
    }
    // 5840
    #[test]
    fn t_5840_ju_test(){
        assert_eq!(test_ju_money_number(5840),0);
    }
    #[test]
    fn t_5840_goju_test(){
        assert_eq!(test_goju_money_number(5840),4);
    }
    #[test]
    fn t_5840_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5840),-16);
    }
    #[test]
    fn t_5840_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5840),-6);
    }
    #[test]
    fn t_5840_sen_test(){
        assert_eq!(test_sen_money_number(5840),6);
    }
    #[test]
    fn t_5840_gosen_test(){
        assert_eq!(test_gosen_money_number(5840),-8);
    }
    #[test]
    fn t_5840_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5840),-2);
    }
    // 5850
    #[test]
    fn t_5850_ju_test(){
        assert_eq!(test_ju_money_number(5850),0);
    }
    #[test]
    fn t_5850_goju_test(){
        assert_eq!(test_goju_money_number(5850),6);
    }
    #[test]
    fn t_5850_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5850),-8);
    }
    #[test]
    fn t_5850_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5850),-4);
    }
    #[test]
    fn t_5850_sen_test(){
        assert_eq!(test_sen_money_number(5850),14);
    }
    #[test]
    fn t_5850_gosen_test(){
        assert_eq!(test_gosen_money_number(5850),-6);
    }
    #[test]
    fn t_5850_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5850),-4);
    }
    // 5860
    #[test]
    fn t_5860_ju_test(){
        assert_eq!(test_ju_money_number(5860),30);
    }
    #[test]
    fn t_5860_goju_test(){
        assert_eq!(test_goju_money_number(5860),-2);
    }
    #[test]
    fn t_5860_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5860),-8);
    }
    #[test]
    fn t_5860_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5860),-4);
    }
    #[test]
    fn t_5860_sen_test(){
        assert_eq!(test_sen_money_number(5860),14);
    }
    #[test]
    fn t_5860_gosen_test(){
        assert_eq!(test_gosen_money_number(5860),-6);
    }
    #[test]
    fn t_5860_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5860),-4);
    }
    // 5870
    #[test]
    fn t_5870_ju_test(){
        assert_eq!(test_ju_money_number(5870),20);
    }
    #[test]
    fn t_5870_goju_test(){
        assert_eq!(test_goju_money_number(5870),-2);
    }
    #[test]
    fn t_5870_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5870),-8);
    }
    #[test]
    fn t_5870_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5870),-4);
    }
    #[test]
    fn t_5870_sen_test(){
        assert_eq!(test_sen_money_number(5870),14);
    }
    #[test]
    fn t_5870_gosen_test(){
        assert_eq!(test_gosen_money_number(5870),-6);
    }
    #[test]
    fn t_5870_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5870),-4);
    }
    // 5890
    #[test]
    fn t_5890_ju_test(){
        assert_eq!(test_ju_money_number(5890),0);
    }
    #[test]
    fn t_5890_goju_test(){
        assert_eq!(test_goju_money_number(5890),-2);
    }
    #[test]
    fn t_5890_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5890),-8);
    }
    #[test]
    fn t_5890_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5890),-4);
    }
    #[test]
    fn t_5890_sen_test(){
        assert_eq!(test_sen_money_number(5890),14);
    }
    #[test]
    fn t_5890_gosen_test(){
        assert_eq!(test_gosen_money_number(5890),-6);
    }
    #[test]
    fn t_5890_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5890),-4);
    }
    // 5900
    #[test]
    fn t_5900_ju_test(){
        assert_eq!(test_ju_money_number(5900),0);
    }
    #[test]
    fn t_5900_goju_test(){
        assert_eq!(test_goju_money_number(5900),0);
    }
    #[test]
    fn t_5900_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5900),-5);
    }
    #[test]
    fn t_5900_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5900),-3);
    }
    #[test]
    fn t_5900_sen_test(){
        assert_eq!(test_sen_money_number(5900),13);
    }
    #[test]
    fn t_5900_gosen_test(){
        assert_eq!(test_gosen_money_number(5900),-6);
    }
    #[test]
    fn t_5900_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5900),-4);
    }
    // 5910
    #[test]
    fn t_5910_ju_test(){
        assert_eq!(test_ju_money_number(5910),30);
    }
    #[test]
    fn t_5910_goju_test(){
        assert_eq!(test_goju_money_number(5910),4);
    }
    #[test]
    fn t_5910_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5910),-16);
    }
    #[test]
    fn t_5910_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5910),-4);
    }
    #[test]
    fn t_5910_sen_test(){
        assert_eq!(test_sen_money_number(5910),14);
    }
    #[test]
    fn t_5910_gosen_test(){
        assert_eq!(test_gosen_money_number(5910),-6);
    }
    #[test]
    fn t_5910_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5910),-4);
    }
    // 5920
    #[test]
    fn t_5920_ju_test(){
        assert_eq!(test_ju_money_number(5920),20);
    }
    #[test]
    fn t_5920_goju_test(){
        assert_eq!(test_goju_money_number(5920),4);
    }
    #[test]
    fn t_5920_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5920),-16);
    }
    #[test]
    fn t_5920_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5920),-4);
    }
    #[test]
    fn t_5920_sen_test(){
        assert_eq!(test_sen_money_number(5920),14);
    }
    #[test]
    fn t_5920_gosen_test(){
        assert_eq!(test_gosen_money_number(5920),-6);
    }
    #[test]
    fn t_5920_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5920),-4);
    }
    // 5930
    #[test]
    fn t_5930_ju_test(){
        assert_eq!(test_ju_money_number(5930),10);
    }
    #[test]
    fn t_5930_goju_test(){
        assert_eq!(test_goju_money_number(5930),4);
    }
    #[test]
    fn t_5930_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5930),-16);
    }
    #[test]
    fn t_5930_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5930),-4);
    }
    #[test]
    fn t_5930_sen_test(){
        assert_eq!(test_sen_money_number(5930),14);
    }
    #[test]
    fn t_5930_gosen_test(){
        assert_eq!(test_gosen_money_number(5930),-6);
    }
    #[test]
    fn t_5930_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5930),-4);
    }
    // 5940
    #[test]
    fn t_5940_ju_test(){
        assert_eq!(test_ju_money_number(5940),0);
    }
    #[test]
    fn t_5940_goju_test(){
        assert_eq!(test_goju_money_number(5940),4);
    }
    #[test]
    fn t_5940_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5940),-16);
    }
    #[test]
    fn t_5940_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5940),-4);
    }
    #[test]
    fn t_5940_sen_test(){
        assert_eq!(test_sen_money_number(5940),14);
    }
    #[test]
    fn t_5940_gosen_test(){
        assert_eq!(test_gosen_money_number(5940),-6);
    }
    #[test]
    fn t_5940_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5940),-4);
    }
    // 5950
    #[test]
    fn t_5950_ju_test(){
        assert_eq!(test_ju_money_number(5950),0);
    }
    #[test]
    fn t_5950_goju_test(){
        assert_eq!(test_goju_money_number(5950),4);
    }
    #[test]
    fn t_5950_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5950),-12);
    }
    #[test]
    fn t_5950_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5950),-3);
    }
    #[test]
    fn t_5950_sen_test(){
        assert_eq!(test_sen_money_number(5950),13);
    }
    #[test]
    fn t_5950_gosen_test(){
        assert_eq!(test_gosen_money_number(5950),-6);
    }
    #[test]
    fn t_5950_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5950),-4);
    }
    // 5960
    #[test]
    fn t_5960_ju_test(){
        assert_eq!(test_ju_money_number(5960),25);
    }
    #[test]
    fn t_5960_goju_test(){
        assert_eq!(test_goju_money_number(5960),-3);
    }
    #[test]
    fn t_5960_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5960),-12);
    }
    #[test]
    fn t_5960_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5960),-3);
    }
    #[test]
    fn t_5960_sen_test(){
        assert_eq!(test_sen_money_number(5960),13);
    }
    #[test]
    fn t_5960_gosen_test(){
        assert_eq!(test_gosen_money_number(5960),-6);
    }
    #[test]
    fn t_5960_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5960),-4);
    }
    // 5970
    #[test]
    fn t_5970_ju_test(){
        assert_eq!(test_ju_money_number(5970),15);
    }
    #[test]
    fn t_5970_goju_test(){
        assert_eq!(test_goju_money_number(5970),-3);
    }
    #[test]
    fn t_5970_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5970),-12);
    }
    #[test]
    fn t_5970_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5970),-3);
    }
    #[test]
    fn t_5970_sen_test(){
        assert_eq!(test_sen_money_number(5970),13);
    }
    #[test]
    fn t_5970_gosen_test(){
        assert_eq!(test_gosen_money_number(5970),-6);
    }
    #[test]
    fn t_5970_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5970),-4);
    }
    // 5980
    #[test]
    fn t_5980_ju_test(){
        assert_eq!(test_ju_money_number(5980),5);
    }
    #[test]
    fn t_5980_goju_test(){
        assert_eq!(test_goju_money_number(5980),-3);
    }
    #[test]
    fn t_5980_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5980),-12);
    }
    #[test]
    fn t_5980_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5980),-3);
    }
    #[test]
    fn t_5980_sen_test(){
        assert_eq!(test_sen_money_number(5980),13);
    }
    #[test]
    fn t_5980_gosen_test(){
        assert_eq!(test_gosen_money_number(5980),-6);
    }
    #[test]
    fn t_5980_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5980),-4);
    }
    // 5990
    #[test]
    fn t_5990_ju_test(){
        assert_eq!(test_ju_money_number(5990),-5);
    }
    #[test]
    fn t_5990_goju_test(){
        assert_eq!(test_goju_money_number(5990),-3);
    }
    #[test]
    fn t_5990_hyaku_test(){
        assert_eq!(test_hyaku_money_number(5990),-12);
    }
    #[test]
    fn t_5990_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(5990),-3);
    }
    #[test]
    fn t_5990_sen_test(){
        assert_eq!(test_sen_money_number(5990),13);
    }
    #[test]
    fn t_5990_gosen_test(){
        assert_eq!(test_gosen_money_number(5990),-6);
    }
    #[test]
    fn t_5990_ichiman_test(){
        assert_eq!(test_ichiman_money_number(5990),-4);
    }
    // 6000
    #[test]
    fn t_6000_ju_test(){
        assert_eq!(test_ju_money_number(6000),0);
    }
    #[test]
    fn t_6000_goju_test(){
        assert_eq!(test_goju_money_number(6000),0);
    }
    #[test]
    fn t_6000_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6000),0);
    }
    #[test]
    fn t_6000_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6000),0);
    }
    #[test]
    fn t_6000_sen_test(){
        assert_eq!(test_sen_money_number(6000),15);
    }
    #[test]
    fn t_6000_gosen_test(){
        assert_eq!(test_gosen_money_number(6000),-5);
    }
    #[test]
    fn t_6000_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6000),-5);
    }
    // 6010
    #[test]
    fn t_6010_ju_test(){
        assert_eq!(test_ju_money_number(6010),35);
    }
    #[test]
    fn t_6010_goju_test(){
        assert_eq!(test_goju_money_number(6010),7);
    }
    #[test]
    fn t_6010_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6010),27);
    }
    #[test]
    fn t_6010_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6010),5);
    }
    #[test]
    fn t_6010_sen_test(){
        assert_eq!(test_sen_money_number(6010),9);
    }
    #[test]
    fn t_6010_gosen_test(){
        assert_eq!(test_gosen_money_number(6010),-5);
    }
    #[test]
    fn t_6010_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6010),-5);
    }
    // 6020
    #[test]
    fn t_6020_ju_test(){
        assert_eq!(test_ju_money_number(6020),25);
    }
    #[test]
    fn t_6020_goju_test(){
        assert_eq!(test_goju_money_number(6020),7);
    }
    #[test]
    fn t_6020_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6020),27);
    }
    #[test]
    fn t_6020_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6020),5);
    }
    #[test]
    fn t_6020_sen_test(){
        assert_eq!(test_sen_money_number(6020),9);
    }
    #[test]
    fn t_6020_gosen_test(){
        assert_eq!(test_gosen_money_number(6020),-5);
    }
    #[test]
    fn t_6020_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6020),-5);
    }
    // 6030
    #[test]
    fn t_6030_ju_test(){
        assert_eq!(test_ju_money_number(6030),15);
    }
    #[test]
    fn t_6030_goju_test(){
        assert_eq!(test_goju_money_number(6030),7);
    }
    #[test]
    fn t_6030_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6030),27);
    }
    #[test]
    fn t_6030_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6030),5);
    }
    #[test]
    fn t_6030_sen_test(){
        assert_eq!(test_sen_money_number(6030),9);
    }
    #[test]
    fn t_6030_gosen_test(){
        assert_eq!(test_gosen_money_number(6030),-5);
    }
    #[test]
    fn t_6030_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6030),-5);
    }
    // 6040
    #[test]
    fn t_6040_ju_test(){
        assert_eq!(test_ju_money_number(6040),5);
    }
    #[test]
    fn t_6040_goju_test(){
        assert_eq!(test_goju_money_number(6040),7);
    }
    #[test]
    fn t_6040_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6040),27);
    }
    #[test]
    fn t_6040_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6040),5);
    }
    #[test]
    fn t_6040_sen_test(){
        assert_eq!(test_sen_money_number(6040),9);
    }
    #[test]
    fn t_6040_gosen_test(){
        assert_eq!(test_gosen_money_number(6040),-5);
    }
    #[test]
    fn t_6040_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6040),-5);
    }
    // 6050
    #[test]
    fn t_6050_ju_test(){
        assert_eq!(test_ju_money_number(6050),0);
    }
    #[test]
    fn t_6050_goju_test(){
        assert_eq!(test_goju_money_number(6050),6);
    }
    #[test]
    fn t_6050_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6050),22);
    }
    #[test]
    fn t_6050_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6050),2);
    }
    #[test]
    fn t_6050_sen_test(){
        assert_eq!(test_sen_money_number(6050),-4);
    }
    #[test]
    fn t_6050_gosen_test(){
        assert_eq!(test_gosen_money_number(6050),-8);
    }
    #[test]
    fn t_6050_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6050),-2);
    }
    // 6060
    #[test]
    fn t_6060_ju_test(){
        assert_eq!(test_ju_money_number(6060),30);
    }
    #[test]
    fn t_6060_goju_test(){
        assert_eq!(test_goju_money_number(6060),-2);
    }
    #[test]
    fn t_6060_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6060),22);
    }
    #[test]
    fn t_6060_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6060),2);
    }
    #[test]
    fn t_6060_sen_test(){
        assert_eq!(test_sen_money_number(6060),-4);
    }
    #[test]
    fn t_6060_gosen_test(){
        assert_eq!(test_gosen_money_number(6060),-8);
    }
    #[test]
    fn t_6060_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6060),-2);
    }
    // 6070
    #[test]
    fn t_6070_ju_test(){
        assert_eq!(test_ju_money_number(6070),20);
    }
    #[test]
    fn t_6070_goju_test(){
        assert_eq!(test_goju_money_number(6070),-2);
    }
    #[test]
    fn t_6070_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6070),22);
    }
    #[test]
    fn t_6070_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6070),2);
    }
    #[test]
    fn t_6070_sen_test(){
        assert_eq!(test_sen_money_number(6070),-4);
    }
    #[test]
    fn t_6070_gosen_test(){
        assert_eq!(test_gosen_money_number(6070),-8);
    }
    #[test]
    fn t_6070_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6070),-2);
    }
    // 6080
    #[test]
    fn t_6080_ju_test(){
        assert_eq!(test_ju_money_number(6080),10);
    }
    #[test]
    fn t_6080_goju_test(){
        assert_eq!(test_goju_money_number(6080),-2);
    }
    #[test]
    fn t_6080_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6080),22);
    }
    #[test]
    fn t_6080_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6080),2);
    }
    #[test]
    fn t_6080_sen_test(){
        assert_eq!(test_sen_money_number(6080),-4);
    }
    #[test]
    fn t_6080_gosen_test(){
        assert_eq!(test_gosen_money_number(6080),-8);
    }
    #[test]
    fn t_6080_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6080),-2);
    }
    // 6090
    #[test]
    fn t_6090_ju_test(){
        assert_eq!(test_ju_money_number(6090),0);
    }
    #[test]
    fn t_6090_goju_test(){
        assert_eq!(test_goju_money_number(6090),-2);
    }
    #[test]
    fn t_6090_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6090),22);
    }
    #[test]
    fn t_6090_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6090),2);
    }
    #[test]
    fn t_6090_sen_test(){
        assert_eq!(test_sen_money_number(6090),-4);
    }
    #[test]
    fn t_6090_gosen_test(){
        assert_eq!(test_gosen_money_number(6090),-8);
    }
    #[test]
    fn t_6090_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6090),-2);
    }
    // 6100
    #[test]
    fn t_6100_ju_test(){
        assert_eq!(test_ju_money_number(6100),0);
    }
    #[test]
    fn t_6100_goju_test(){
        assert_eq!(test_goju_money_number(6100),0);
    }
    #[test]
    fn t_6100_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6100),30);
    }
    #[test]
    fn t_6100_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6100),4);
    }
    #[test]
    fn t_6100_sen_test(){
        assert_eq!(test_sen_money_number(6100),4);
    }
    #[test]
    fn t_6100_gosen_test(){
        assert_eq!(test_gosen_money_number(6100),-6);
    }
    #[test]
    fn t_6100_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6100),-4);
    }
    // 6200
    #[test]
    fn t_6200_ju_test(){
        assert_eq!(test_ju_money_number(6200),0);
    }
    #[test]
    fn t_6200_goju_test(){
        assert_eq!(test_goju_money_number(6200),0);
    }
    #[test]
    fn t_6200_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6200),20);
    }
    #[test]
    fn t_6200_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6200),4);
    }
    #[test]
    fn t_6200_sen_test(){
        assert_eq!(test_sen_money_number(6200),4);
    }
    #[test]
    fn t_6200_gosen_test(){
        assert_eq!(test_gosen_money_number(6200),-6);
    }
    #[test]
    fn t_6200_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6200),-4);
    }
    // 6210
    #[test]
    fn t_6210_ju_test(){
        assert_eq!(test_ju_money_number(6210),35);
    }
    #[test]
    fn t_6210_goju_test(){
        assert_eq!(test_goju_money_number(6210),7);
    }
    #[test]
    fn t_6210_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6210),7);
    }
    #[test]
    fn t_6210_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6210),5);
    }
    #[test]
    fn t_6210_sen_test(){
        assert_eq!(test_sen_money_number(6210),9);
    }
    #[test]
    fn t_6210_gosen_test(){
        assert_eq!(test_gosen_money_number(6210),-5);
    }
    #[test]
    fn t_6210_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6210),-5);
    }
    // 6220
    #[test]
    fn t_6220_ju_test(){
        assert_eq!(test_ju_money_number(6220),25);
    }
    #[test]
    fn t_6220_goju_test(){
        assert_eq!(test_goju_money_number(6220),7);
    }
    #[test]
    fn t_6220_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6220),7);
    }
    #[test]
    fn t_6220_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6220),5);
    }
    #[test]
    fn t_6220_sen_test(){
        assert_eq!(test_sen_money_number(6220),9);
    }
    #[test]
    fn t_6220_gosen_test(){
        assert_eq!(test_gosen_money_number(6220),-5);
    }
    #[test]
    fn t_6220_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6220),-5);
    }
    // 6230
    #[test]
    fn t_6230_ju_test(){
        assert_eq!(test_ju_money_number(6230),15);
    }
    #[test]
    fn t_6230_goju_test(){
        assert_eq!(test_goju_money_number(6230),7);
    }
    #[test]
    fn t_6230_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6230),7);
    }
    #[test]
    fn t_6230_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6230),5);
    }
    #[test]
    fn t_6230_sen_test(){
        assert_eq!(test_sen_money_number(6230),9);
    }
    #[test]
    fn t_6230_gosen_test(){
        assert_eq!(test_gosen_money_number(6230),-5);
    }
    #[test]
    fn t_6230_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6230),-5);
    }
    // 6240
    #[test]
    fn t_6240_ju_test(){
        assert_eq!(test_ju_money_number(6240),5);
    }
    #[test]
    fn t_6240_goju_test(){
        assert_eq!(test_goju_money_number(6240),7);
    }
    #[test]
    fn t_6240_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6240),7);
    }
    #[test]
    fn t_6240_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6240),5);
    }
    #[test]
    fn t_6240_sen_test(){
        assert_eq!(test_sen_money_number(6240),9);
    }
    #[test]
    fn t_6240_gosen_test(){
        assert_eq!(test_gosen_money_number(6240),-5);
    }
    #[test]
    fn t_6240_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6240),-5);
    }
    // 6250
    #[test]
    fn t_6250_ju_test(){
        assert_eq!(test_ju_money_number(6250),0);
    }
    #[test]
    fn t_6250_goju_test(){
        assert_eq!(test_goju_money_number(6250),6);
    }
    #[test]
    fn t_6250_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6250),2);
    }
    #[test]
    fn t_6250_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6250),2);
    }
    #[test]
    fn t_6250_sen_test(){
        assert_eq!(test_sen_money_number(6250),-4);
    }
    #[test]
    fn t_6250_gosen_test(){
        assert_eq!(test_gosen_money_number(6250),-8);
    }
    #[test]
    fn t_6250_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6250),-2);
    }
    // 6260
    #[test]
    fn t_6260_ju_test(){
        assert_eq!(test_ju_money_number(6260),30);
    }
    #[test]
    fn t_6260_goju_test(){
        assert_eq!(test_goju_money_number(6260),-2);
    }
    #[test]
    fn t_6260_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6260),2);
    }
    #[test]
    fn t_6260_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6260),2);
    }
    #[test]
    fn t_6260_sen_test(){
        assert_eq!(test_sen_money_number(6260),-4);
    }
    #[test]
    fn t_6260_gosen_test(){
        assert_eq!(test_gosen_money_number(6260),-8);
    }
    #[test]
    fn t_6260_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6260),-2);
    }
    // 6270
    #[test]
    fn t_6270_ju_test(){
        assert_eq!(test_ju_money_number(6270),20);
    }
    #[test]
    fn t_6270_goju_test(){
        assert_eq!(test_goju_money_number(6270),-2);
    }
    #[test]
    fn t_6270_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6270),2);
    }
    #[test]
    fn t_6270_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6270),2);
    }
    #[test]
    fn t_6270_sen_test(){
        assert_eq!(test_sen_money_number(6270),-4);
    }
    #[test]
    fn t_6270_gosen_test(){
        assert_eq!(test_gosen_money_number(6270),-8);
    }
    #[test]
    fn t_6270_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6270),-2);
    }
    // 6280
    #[test]
    fn t_6280_ju_test(){
        assert_eq!(test_ju_money_number(6280),10);
    }
    #[test]
    fn t_6280_goju_test(){
        assert_eq!(test_goju_money_number(6280),-2);
    }
    #[test]
    fn t_6280_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6280),2);
    }
    #[test]
    fn t_6280_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6280),2);
    }
    #[test]
    fn t_6280_sen_test(){
        assert_eq!(test_sen_money_number(6280),-4);
    }
    #[test]
    fn t_6280_gosen_test(){
        assert_eq!(test_gosen_money_number(6280),-8);
    }
    #[test]
    fn t_6280_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6280),-2);
    }
    // 6290
    #[test]
    fn t_6290_ju_test(){
        assert_eq!(test_ju_money_number(6290),0);
    }
    #[test]
    fn t_6290_goju_test(){
        assert_eq!(test_goju_money_number(6290),-2);
    }
    #[test]
    fn t_6290_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6290),2);
    }
    #[test]
    fn t_6290_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6290),2);
    }
    #[test]
    fn t_6290_sen_test(){
        assert_eq!(test_sen_money_number(6290),-4);
    }
    #[test]
    fn t_6290_gosen_test(){
        assert_eq!(test_gosen_money_number(6290),-8);
    }
    #[test]
    fn t_6290_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6290),-2);
    }
    // 6300
    #[test]
    fn t_6300_ju_test(){
        assert_eq!(test_ju_money_number(6300),0);
    }
    #[test]
    fn t_6300_goju_test(){
        assert_eq!(test_goju_money_number(6300),0);
    }
    #[test]
    fn t_6300_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6300),10);
    }
    #[test]
    fn t_6300_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6300),4);
    }
    #[test]
    fn t_6300_sen_test(){
        assert_eq!(test_sen_money_number(6300),4);
    }
    #[test]
    fn t_6300_gosen_test(){
        assert_eq!(test_gosen_money_number(6300),-6);
    }
    #[test]
    fn t_6300_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6300),-4);
    }
    // 6310
    #[test]
    fn t_6310_ju_test(){
        assert_eq!(test_ju_money_number(6310),35);
    }
    #[test]
    fn t_6310_goju_test(){
        assert_eq!(test_goju_money_number(6310),7);
    }
    #[test]
    fn t_6310_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6310),-3);
    }
    #[test]
    fn t_6310_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6310),5);
    }
    #[test]
    fn t_6310_sen_test(){
        assert_eq!(test_sen_money_number(6310),9);
    }
    #[test]
    fn t_6310_gosen_test(){
        assert_eq!(test_gosen_money_number(6310),-5);
    }
    #[test]
    fn t_6310_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6310),-5);
    }
    // 6320
    #[test]
    fn t_6320_ju_test(){
        assert_eq!(test_ju_money_number(6320),25);
    }
    #[test]
    fn t_6320_goju_test(){
        assert_eq!(test_goju_money_number(6320),7);
    }
    #[test]
    fn t_6320_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6320),-3);
    }
    #[test]
    fn t_6320_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6320),5);
    }
    #[test]
    fn t_6320_sen_test(){
        assert_eq!(test_sen_money_number(6320),9);
    }
    #[test]
    fn t_6320_gosen_test(){
        assert_eq!(test_gosen_money_number(6320),-5);
    }
    #[test]
    fn t_6320_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6320),-5);
    }
    // 6330
    #[test]
    fn t_6330_ju_test(){
        assert_eq!(test_ju_money_number(6330),15);
    }
    #[test]
    fn t_6330_goju_test(){
        assert_eq!(test_goju_money_number(6330),7);
    }
    #[test]
    fn t_6330_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6330),-3);
    }
    #[test]
    fn t_6330_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6330),5);
    }
    #[test]
    fn t_6330_sen_test(){
        assert_eq!(test_sen_money_number(6330),9);
    }
    #[test]
    fn t_6330_gosen_test(){
        assert_eq!(test_gosen_money_number(6330),-5);
    }
    #[test]
    fn t_6330_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6330),-5);
    }
    // 6340
    #[test]
    fn t_6340_ju_test(){
        assert_eq!(test_ju_money_number(6340),5);
    }
    #[test]
    fn t_6340_goju_test(){
        assert_eq!(test_goju_money_number(6340),7);
    }
    #[test]
    fn t_6340_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6340),-3);
    }
    #[test]
    fn t_6340_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6340),5);
    }
    #[test]
    fn t_6340_sen_test(){
        assert_eq!(test_sen_money_number(6340),9);
    }
    #[test]
    fn t_6340_gosen_test(){
        assert_eq!(test_gosen_money_number(6340),-5);
    }
    #[test]
    fn t_6340_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6340),-5);
    }
    // 6350
    #[test]
    fn t_6350_ju_test(){
        assert_eq!(test_ju_money_number(6350),0);
    }
    #[test]
    fn t_6350_goju_test(){
        assert_eq!(test_goju_money_number(6350),6);
    }
    #[test]
    fn t_6350_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6350),-8);
    }
    #[test]
    fn t_6350_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6350),2);
    }
    #[test]
    fn t_6350_sen_test(){
        assert_eq!(test_sen_money_number(6350),-4);
    }
    #[test]
    fn t_6350_gosen_test(){
        assert_eq!(test_gosen_money_number(6350),-8);
    }
    #[test]
    fn t_6350_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6350),-2);
    }
    // 6360
    #[test]
    fn t_6360_ju_test(){
        assert_eq!(test_ju_money_number(6360),30);
    }
    #[test]
    fn t_6360_goju_test(){
        assert_eq!(test_goju_money_number(6360),-2);
    }
    #[test]
    fn t_6360_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6360),-8);
    }
    #[test]
    fn t_6360_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6360),2);
    }
    #[test]
    fn t_6360_sen_test(){
        assert_eq!(test_sen_money_number(6360),-4);
    }
    #[test]
    fn t_6360_gosen_test(){
        assert_eq!(test_gosen_money_number(6360),-8);
    }
    #[test]
    fn t_6360_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6360),-2);
    }
    // 6370
    #[test]
    fn t_6370_ju_test(){
        assert_eq!(test_ju_money_number(6370),20);
    }
    #[test]
    fn t_6370_goju_test(){
        assert_eq!(test_goju_money_number(6370),-2);
    }
    #[test]
    fn t_6370_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6370),-8);
    }
    #[test]
    fn t_6370_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6370),2);
    }
    #[test]
    fn t_6370_sen_test(){
        assert_eq!(test_sen_money_number(6370),-4);
    }
    #[test]
    fn t_6370_gosen_test(){
        assert_eq!(test_gosen_money_number(6370),-8);
    }
    #[test]
    fn t_6370_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6370),-2);
    }
    // 6380
    #[test]
    fn t_6380_ju_test(){
        assert_eq!(test_ju_money_number(6380),10);
    }
    #[test]
    fn t_6380_goju_test(){
        assert_eq!(test_goju_money_number(6380),-2);
    }
    #[test]
    fn t_6380_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6380),-8);
    }
    #[test]
    fn t_6380_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6380),2);
    }
    #[test]
    fn t_6380_sen_test(){
        assert_eq!(test_sen_money_number(6380),-4);
    }
    #[test]
    fn t_6380_gosen_test(){
        assert_eq!(test_gosen_money_number(6380),-8);
    }
    #[test]
    fn t_6380_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6380),-2);
    }
    // 6390
    #[test]
    fn t_6390_ju_test(){
        assert_eq!(test_ju_money_number(6390),0);
    }
    #[test]
    fn t_6390_goju_test(){
        assert_eq!(test_goju_money_number(6390),-2);
    }
    #[test]
    fn t_6390_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6390),-8);
    }
    #[test]
    fn t_6390_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6390),2);
    }
    #[test]
    fn t_6390_sen_test(){
        assert_eq!(test_sen_money_number(6390),-4);
    }
    #[test]
    fn t_6390_gosen_test(){
        assert_eq!(test_gosen_money_number(6390),-8);
    }
    #[test]
    fn t_6390_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6390),-2);
    }
    // 6400
    #[test]
    fn t_6400_ju_test(){
        assert_eq!(test_ju_money_number(6400),0);
    }
    #[test]
    fn t_6400_goju_test(){
        assert_eq!(test_goju_money_number(6400),0);
    }
    #[test]
    fn t_6400_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6400),0);
    }
    #[test]
    fn t_6400_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6400),4);
    }
    #[test]
    fn t_6400_sen_test(){
        assert_eq!(test_sen_money_number(6400),4);
    }
    #[test]
    fn t_6400_gosen_test(){
        assert_eq!(test_gosen_money_number(6400),-6);
    }
    #[test]
    fn t_6400_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6400),-4);
    }
    // 6410
    #[test]
    fn t_6410_ju_test(){
        assert_eq!(test_ju_money_number(6410),30);
    }
    #[test]
    fn t_6410_goju_test(){
        assert_eq!(test_goju_money_number(6410),4);
    }
    #[test]
    fn t_6410_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6410),-16);
    }
    #[test]
    fn t_6410_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6410),2);
    }
    #[test]
    fn t_6410_sen_test(){
        assert_eq!(test_sen_money_number(6410),-4);
    }
    #[test]
    fn t_6410_gosen_test(){
        assert_eq!(test_gosen_money_number(6410),-8);
    }
    #[test]
    fn t_6410_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6410),-2);
    }
    // 6420
    #[test]
    fn t_6420_ju_test(){
        assert_eq!(test_ju_money_number(6420),20);
    }
    #[test]
    fn t_6420_goju_test(){
        assert_eq!(test_goju_money_number(6420),4);
    }
    #[test]
    fn t_6420_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6420),-16);
    }
    #[test]
    fn t_6420_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6420),2);
    }
    #[test]
    fn t_6420_sen_test(){
        assert_eq!(test_sen_money_number(6420),-4);
    }
    #[test]
    fn t_6420_gosen_test(){
        assert_eq!(test_gosen_money_number(6420),-8);
    }
    #[test]
    fn t_6420_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6420),-2);
    }
    // 6430
    #[test]
    fn t_6430_ju_test(){
        assert_eq!(test_ju_money_number(6430),10);
    }
    #[test]
    fn t_6430_goju_test(){
        assert_eq!(test_goju_money_number(6430),4);
    }
    #[test]
    fn t_6430_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6430),-16);
    }
    #[test]
    fn t_6430_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6430),2);
    }
    #[test]
    fn t_6430_sen_test(){
        assert_eq!(test_sen_money_number(6430),-4);
    }
    #[test]
    fn t_6430_gosen_test(){
        assert_eq!(test_gosen_money_number(6430),-8);
    }
    #[test]
    fn t_6430_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6430),-2);
    }
    // 6440
    #[test]
    fn t_6440_ju_test(){
        assert_eq!(test_ju_money_number(6440),0);
    }
    #[test]
    fn t_6440_goju_test(){
        assert_eq!(test_goju_money_number(6440),4);
    }
    #[test]
    fn t_6440_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6440),-16);
    }
    #[test]
    fn t_6440_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6440),2);
    }
    #[test]
    fn t_6440_sen_test(){
        assert_eq!(test_sen_money_number(6440),-4);
    }
    #[test]
    fn t_6440_gosen_test(){
        assert_eq!(test_gosen_money_number(6440),-8);
    }
    #[test]
    fn t_6440_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6440),-2);
    }
    // 6450
    #[test]
    fn t_6450_ju_test(){
        assert_eq!(test_ju_money_number(6450),0);
    }
    #[test]
    fn t_6450_goju_test(){
        assert_eq!(test_goju_money_number(6450),6);
    }
    #[test]
    fn t_6450_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6450),-8);
    }
    #[test]
    fn t_6450_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6450),4);
    }
    #[test]
    fn t_6450_sen_test(){
        assert_eq!(test_sen_money_number(6450),4);
    }
    #[test]
    fn t_6450_gosen_test(){
        assert_eq!(test_gosen_money_number(6450),-6);
    }
    #[test]
    fn t_6450_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6450),-4);
    }
    // 6460
    #[test]
    fn t_6460_ju_test(){
        assert_eq!(test_ju_money_number(6460),30);
    }
    #[test]
    fn t_6460_goju_test(){
        assert_eq!(test_goju_money_number(6460),-2);
    }
    #[test]
    fn t_6460_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6460),-8);
    }
    #[test]
    fn t_6460_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6460),4);
    }
    #[test]
    fn t_6460_sen_test(){
        assert_eq!(test_sen_money_number(6460),4);
    }
    #[test]
    fn t_6460_gosen_test(){
        assert_eq!(test_gosen_money_number(6460),-6);
    }
    #[test]
    fn t_6460_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6460),-4);
    }
    // 6470
    #[test]
    fn t_6470_ju_test(){
        assert_eq!(test_ju_money_number(6470),20);
    }
    #[test]
    fn t_6470_goju_test(){
        assert_eq!(test_goju_money_number(6470),-2);
    }
    #[test]
    fn t_6470_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6470),-8);
    }
    #[test]
    fn t_6470_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6470),4);
    }
    #[test]
    fn t_6470_sen_test(){
        assert_eq!(test_sen_money_number(6470),4);
    }
    #[test]
    fn t_6470_gosen_test(){
        assert_eq!(test_gosen_money_number(6470),-6);
    }
    #[test]
    fn t_6470_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6470),-4);
    }
    // 6480
    #[test]
    fn t_6480_ju_test(){
        assert_eq!(test_ju_money_number(6480),10);
    }
    #[test]
    fn t_6480_goju_test(){
        assert_eq!(test_goju_money_number(6480),-2);
    }
    #[test]
    fn t_6480_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6480),-8);
    }
    #[test]
    fn t_6480_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6480),4);
    }
    #[test]
    fn t_6480_sen_test(){
        assert_eq!(test_sen_money_number(6480),4);
    }
    #[test]
    fn t_6480_gosen_test(){
        assert_eq!(test_gosen_money_number(6480),-6);
    }
    #[test]
    fn t_6480_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6480),-4);
    }
    // 6490
    #[test]
    fn t_6490_ju_test(){
        assert_eq!(test_ju_money_number(6490),0);
    }
    #[test]
    fn t_6490_goju_test(){
        assert_eq!(test_goju_money_number(6490),-2);
    }
    #[test]
    fn t_6490_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6490),-8);
    }
    #[test]
    fn t_6490_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6490),4);
    }
    #[test]
    fn t_6490_sen_test(){
        assert_eq!(test_sen_money_number(6490),4);
    }
    #[test]
    fn t_6490_gosen_test(){
        assert_eq!(test_gosen_money_number(6490),-6);
    }
    #[test]
    fn t_6490_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6490),-4);
    }
    // 6500
    #[test]
    fn t_6500_ju_test(){
        assert_eq!(test_ju_money_number(6500),0);
    }
    #[test]
    fn t_6500_goju_test(){
        assert_eq!(test_goju_money_number(6500),0);
    }
    #[test]
    fn t_6500_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6500),0);
    }
    #[test]
    fn t_6500_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6500),4);
    }
    #[test]
    fn t_6500_sen_test(){
        assert_eq!(test_sen_money_number(6500),3);
    }
    #[test]
    fn t_6500_gosen_test(){
        assert_eq!(test_gosen_money_number(6500),-6);
    }
    #[test]
    fn t_6500_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6500),-4);
    }
    // 6510
    #[test]
    fn t_6510_ju_test(){
        assert_eq!(test_ju_money_number(6510),30);
    }
    #[test]
    fn t_6510_goju_test(){
        assert_eq!(test_goju_money_number(6510),4);
    }
    #[test]
    fn t_6510_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6510),14);
    }
    #[test]
    fn t_6510_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6510),-6);
    }
    #[test]
    fn t_6510_sen_test(){
        assert_eq!(test_sen_money_number(6510),-4);
    }
    #[test]
    fn t_6510_gosen_test(){
        assert_eq!(test_gosen_money_number(6510),-8);
    }
    #[test]
    fn t_6510_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6510),-2);
    }
    // 6520
    #[test]
    fn t_6520_ju_test(){
        assert_eq!(test_ju_money_number(6520),20);
    }
    #[test]
    fn t_6520_goju_test(){
        assert_eq!(test_goju_money_number(6520),4);
    }
    #[test]
    fn t_6520_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6520),14);
    }
    #[test]
    fn t_6520_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6520),-6);
    }
    #[test]
    fn t_6520_sen_test(){
        assert_eq!(test_sen_money_number(6520),-4);
    }
    #[test]
    fn t_6520_gosen_test(){
        assert_eq!(test_gosen_money_number(6520),-8);
    }
    #[test]
    fn t_6520_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6520),-2);
    }
    // 6530
    #[test]
    fn t_6530_ju_test(){
        assert_eq!(test_ju_money_number(6530),10);
    }
    #[test]
    fn t_6530_goju_test(){
        assert_eq!(test_goju_money_number(6530),4);
    }
    #[test]
    fn t_6530_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6530),14);
    }
    #[test]
    fn t_6530_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6530),-6);
    }
    #[test]
    fn t_6530_sen_test(){
        assert_eq!(test_sen_money_number(6530),-4);
    }
    #[test]
    fn t_6530_gosen_test(){
        assert_eq!(test_gosen_money_number(6530),-8);
    }
    #[test]
    fn t_6530_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6530),-2);
    }
    // 6540
    #[test]
    fn t_6540_ju_test(){
        assert_eq!(test_ju_money_number(6540),0);
    }
    #[test]
    fn t_6540_goju_test(){
        assert_eq!(test_goju_money_number(6540),4);
    }
    #[test]
    fn t_6540_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6540),14);
    }
    #[test]
    fn t_6540_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6540),-6);
    }
    #[test]
    fn t_6540_sen_test(){
        assert_eq!(test_sen_money_number(6540),-4);
    }
    #[test]
    fn t_6540_gosen_test(){
        assert_eq!(test_gosen_money_number(6540),-8);
    }
    #[test]
    fn t_6540_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6540),-2);
    }
    // 6550
    #[test]
    fn t_6550_ju_test(){
        assert_eq!(test_ju_money_number(6550),0);
    }
    #[test]
    fn t_6550_goju_test(){
        assert_eq!(test_goju_money_number(6550),6);
    }
    #[test]
    fn t_6550_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6550),22);
    }
    #[test]
    fn t_6550_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6550),-4);
    }
    #[test]
    fn t_6550_sen_test(){
        assert_eq!(test_sen_money_number(6550),4);
    }
    #[test]
    fn t_6550_gosen_test(){
        assert_eq!(test_gosen_money_number(6550),-6);
    }
    #[test]
    fn t_6550_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6550),-4);
    }
    // 6560
    #[test]
    fn t_6560_ju_test(){
        assert_eq!(test_ju_money_number(6560),30);
    }
    #[test]
    fn t_6560_goju_test(){
        assert_eq!(test_goju_money_number(6560),-2);
    }
    #[test]
    fn t_6560_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6560),22);
    }
    #[test]
    fn t_6560_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6560),-4);
    }
    #[test]
    fn t_6560_sen_test(){
        assert_eq!(test_sen_money_number(6560),4);
    }
    #[test]
    fn t_6560_gosen_test(){
        assert_eq!(test_gosen_money_number(6560),-6);
    }
    #[test]
    fn t_6560_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6560),-4);
    }
    // 6570
    #[test]
    fn t_6570_ju_test(){
        assert_eq!(test_ju_money_number(6570),20);
    }
    #[test]
    fn t_6570_goju_test(){
        assert_eq!(test_goju_money_number(6570),-2);
    }
    #[test]
    fn t_6570_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6570),22);
    }
    #[test]
    fn t_6570_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6570),-4);
    }
    #[test]
    fn t_6570_sen_test(){
        assert_eq!(test_sen_money_number(6570),4);
    }
    #[test]
    fn t_6570_gosen_test(){
        assert_eq!(test_gosen_money_number(6570),-6);
    }
    #[test]
    fn t_6570_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6570),-4);
    }
    // 6580
    #[test]
    fn t_6580_ju_test(){
        assert_eq!(test_ju_money_number(6580),10);
    }
    #[test]
    fn t_6580_goju_test(){
        assert_eq!(test_goju_money_number(6580),-2);
    }
    #[test]
    fn t_6580_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6580),22);
    }
    #[test]
    fn t_6580_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6580),-4);
    }
    #[test]
    fn t_6580_sen_test(){
        assert_eq!(test_sen_money_number(6580),4);
    }
    #[test]
    fn t_6580_gosen_test(){
        assert_eq!(test_gosen_money_number(6580),-6);
    }
    #[test]
    fn t_6580_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6580),-4);
    }
    // 6590
    #[test]
    fn t_6590_ju_test(){
        assert_eq!(test_ju_money_number(6590),0);
    }
    #[test]
    fn t_6590_goju_test(){
        assert_eq!(test_goju_money_number(6590),-2);
    }
    #[test]
    fn t_6590_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6590),22);
    }
    #[test]
    fn t_6590_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6590),-4);
    }
    #[test]
    fn t_6590_sen_test(){
        assert_eq!(test_sen_money_number(6590),4);
    }
    #[test]
    fn t_6590_gosen_test(){
        assert_eq!(test_gosen_money_number(6590),-6);
    }
    #[test]
    fn t_6590_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6590),-4);
    }
    // 6600
    #[test]
    fn t_6600_ju_test(){
        assert_eq!(test_ju_money_number(6600),0);
    }
    #[test]
    fn t_6600_goju_test(){
        assert_eq!(test_goju_money_number(6600),0);
    }
    #[test]
    fn t_6600_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6600),25);
    }
    #[test]
    fn t_6600_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6600),-3);
    }
    #[test]
    fn t_6600_sen_test(){
        assert_eq!(test_sen_money_number(6600),3);
    }
    #[test]
    fn t_6600_gosen_test(){
        assert_eq!(test_gosen_money_number(6600),-6);
    }
    #[test]
    fn t_6600_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6600),-4);
    }
    // 6610
    #[test]
    fn t_6610_ju_test(){
        assert_eq!(test_ju_money_number(6610),30);
    }
    #[test]
    fn t_6610_goju_test(){
        assert_eq!(test_goju_money_number(6610),4);
    }
    #[test]
    fn t_6610_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6610),4);
    }
    #[test]
    fn t_6610_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6610),-6);
    }
    #[test]
    fn t_6610_sen_test(){
        assert_eq!(test_sen_money_number(6610),-4);
    }
    #[test]
    fn t_6610_gosen_test(){
        assert_eq!(test_gosen_money_number(6610),-8);
    }
    #[test]
    fn t_6610_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6610),-2);
    }
    // 6620
    #[test]
    fn t_6620_ju_test(){
        assert_eq!(test_ju_money_number(6620),20);
    }
    #[test]
    fn t_6620_goju_test(){
        assert_eq!(test_goju_money_number(6620),4);
    }
    #[test]
    fn t_6620_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6620),4);
    }
    #[test]
    fn t_6620_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6620),-6);
    }
    #[test]
    fn t_6620_sen_test(){
        assert_eq!(test_sen_money_number(6620),-4);
    }
    #[test]
    fn t_6620_gosen_test(){
        assert_eq!(test_gosen_money_number(6620),-8);
    }
    #[test]
    fn t_6620_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6620),-2);
    }
    // 6630
    #[test]
    fn t_6630_ju_test(){
        assert_eq!(test_ju_money_number(6630),10);
    }
    #[test]
    fn t_6630_goju_test(){
        assert_eq!(test_goju_money_number(6630),4);
    }
    #[test]
    fn t_6630_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6630),4);
    }
    #[test]
    fn t_6630_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6630),-6);
    }
    #[test]
    fn t_6630_sen_test(){
        assert_eq!(test_sen_money_number(6630),-4);
    }
    #[test]
    fn t_6630_gosen_test(){
        assert_eq!(test_gosen_money_number(6630),-8);
    }
    #[test]
    fn t_6630_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6630),-2);
    }
    // 6640
    #[test]
    fn t_6640_ju_test(){
        assert_eq!(test_ju_money_number(6640),0);
    }
    #[test]
    fn t_6640_goju_test(){
        assert_eq!(test_goju_money_number(6640),4);
    }
    #[test]
    fn t_6640_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6640),4);
    }
    #[test]
    fn t_6640_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6640),-6);
    }
    #[test]
    fn t_6640_sen_test(){
        assert_eq!(test_sen_money_number(6640),-4);
    }
    #[test]
    fn t_6640_gosen_test(){
        assert_eq!(test_gosen_money_number(6640),-8);
    }
    #[test]
    fn t_6640_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6640),-2);
    }
    // 6650
    #[test]
    fn t_6650_ju_test(){
        assert_eq!(test_ju_money_number(6650),0);
    }
    #[test]
    fn t_6650_goju_test(){
        assert_eq!(test_goju_money_number(6650),6);
    }
    #[test]
    fn t_6650_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6650),12);
    }
    #[test]
    fn t_6650_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6650),-4);
    }
    #[test]
    fn t_6650_sen_test(){
        assert_eq!(test_sen_money_number(6650),4);
    }
    #[test]
    fn t_6650_gosen_test(){
        assert_eq!(test_gosen_money_number(6650),-6);
    }
    #[test]
    fn t_6650_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6650),-4);
    }
    // 6660
    #[test]
    fn t_6660_ju_test(){
        assert_eq!(test_ju_money_number(6660),30);
    }
    #[test]
    fn t_6660_goju_test(){
        assert_eq!(test_goju_money_number(6660),-2);
    }
    #[test]
    fn t_6660_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6660),12);
    }
    #[test]
    fn t_6660_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6660),-4);
    }
    #[test]
    fn t_6660_sen_test(){
        assert_eq!(test_sen_money_number(6660),4);
    }
    #[test]
    fn t_6660_gosen_test(){
        assert_eq!(test_gosen_money_number(6660),-6);
    }
    #[test]
    fn t_6660_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6660),-4);
    }
    // 6670
    #[test]
    fn t_6670_ju_test(){
        assert_eq!(test_ju_money_number(6670),20);
    }
    #[test]
    fn t_6670_goju_test(){
        assert_eq!(test_goju_money_number(6670),-2);
    }
    #[test]
    fn t_6670_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6670),12);
    }
    #[test]
    fn t_6670_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6670),-4);
    }
    #[test]
    fn t_6670_sen_test(){
        assert_eq!(test_sen_money_number(6670),4);
    }
    #[test]
    fn t_6670_gosen_test(){
        assert_eq!(test_gosen_money_number(6670),-6);
    }
    #[test]
    fn t_6670_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6670),-4);
    }
    // 6680
    #[test]
    fn t_6680_ju_test(){
        assert_eq!(test_ju_money_number(6680),10);
    }
    #[test]
    fn t_6680_goju_test(){
        assert_eq!(test_goju_money_number(6680),-2);
    }
    #[test]
    fn t_6680_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6680),12);
    }
    #[test]
    fn t_6680_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6680),-4);
    }
    #[test]
    fn t_6680_sen_test(){
        assert_eq!(test_sen_money_number(6680),4);
    }
    #[test]
    fn t_6680_gosen_test(){
        assert_eq!(test_gosen_money_number(6680),-6);
    }
    #[test]
    fn t_6680_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6680),-4);
    }
    // 6690
    #[test]
    fn t_6690_ju_test(){
        assert_eq!(test_ju_money_number(6690),0);
    }
    #[test]
    fn t_6690_goju_test(){
        assert_eq!(test_goju_money_number(6690),-2);
    }
    #[test]
    fn t_6690_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6690),12);
    }
    #[test]
    fn t_6690_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6690),-4);
    }
    #[test]
    fn t_6690_sen_test(){
        assert_eq!(test_sen_money_number(6690),4);
    }
    #[test]
    fn t_6690_gosen_test(){
        assert_eq!(test_gosen_money_number(6690),-6);
    }
    #[test]
    fn t_6690_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6690),-4);
    }
    // 6700
    #[test]
    fn t_6700_ju_test(){
        assert_eq!(test_ju_money_number(6700),0);
    }
    #[test]
    fn t_6700_goju_test(){
        assert_eq!(test_goju_money_number(6700),0);
    }
    #[test]
    fn t_6700_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6700),15);
    }
    #[test]
    fn t_6700_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6700),-3);
    }
    #[test]
    fn t_6700_sen_test(){
        assert_eq!(test_sen_money_number(6700),3);
    }
    #[test]
    fn t_6700_gosen_test(){
        assert_eq!(test_gosen_money_number(6700),-6);
    }
    #[test]
    fn t_6700_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6700),-4);
    }
    // 6710
    #[test]
    fn t_6710_ju_test(){
        assert_eq!(test_ju_money_number(6710),30);
    }
    #[test]
    fn t_6710_goju_test(){
        assert_eq!(test_goju_money_number(6710),4);
    }
    #[test]
    fn t_6710_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6710),-6);
    }
    #[test]
    fn t_6710_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6710),-6);
    }
    #[test]
    fn t_6710_sen_test(){
        assert_eq!(test_sen_money_number(6710),-4);
    }
    #[test]
    fn t_6710_gosen_test(){
        assert_eq!(test_gosen_money_number(6710),-8);
    }
    #[test]
    fn t_6710_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6710),-2);
    }
    // 6720
    #[test]
    fn t_6720_ju_test(){
        assert_eq!(test_ju_money_number(6720),20);
    }
    #[test]
    fn t_6720_goju_test(){
        assert_eq!(test_goju_money_number(6720),4);
    }
    #[test]
    fn t_6720_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6720),-6);
    }
    #[test]
    fn t_6720_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6720),-6);
    }
    #[test]
    fn t_6720_sen_test(){
        assert_eq!(test_sen_money_number(6720),-4);
    }
    #[test]
    fn t_6720_gosen_test(){
        assert_eq!(test_gosen_money_number(6720),-8);
    }
    #[test]
    fn t_6720_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6720),-2);
    }
    // 6730
    #[test]
    fn t_6730_ju_test(){
        assert_eq!(test_ju_money_number(6730),10);
    }
    #[test]
    fn t_6730_goju_test(){
        assert_eq!(test_goju_money_number(6730),4);
    }
    #[test]
    fn t_6730_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6730),-6);
    }
    #[test]
    fn t_6730_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6730),-6);
    }
    #[test]
    fn t_6730_sen_test(){
        assert_eq!(test_sen_money_number(6730),-4);
    }
    #[test]
    fn t_6730_gosen_test(){
        assert_eq!(test_gosen_money_number(6730),-8);
    }
    #[test]
    fn t_6730_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6730),-2);
    }
    // 6740
    #[test]
    fn t_6740_ju_test(){
        assert_eq!(test_ju_money_number(6740),0);
    }
    #[test]
    fn t_6740_goju_test(){
        assert_eq!(test_goju_money_number(6740),4);
    }
    #[test]
    fn t_6740_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6740),-6);
    }
    #[test]
    fn t_6740_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6740),-6);
    }
    #[test]
    fn t_6740_sen_test(){
        assert_eq!(test_sen_money_number(6740),-4);
    }
    #[test]
    fn t_6740_gosen_test(){
        assert_eq!(test_gosen_money_number(6740),-8);
    }
    #[test]
    fn t_6740_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6740),-2);
    }
    // 6750
    #[test]
    fn t_6750_ju_test(){
        assert_eq!(test_ju_money_number(6750),0);
    }
    #[test]
    fn t_6750_goju_test(){
        assert_eq!(test_goju_money_number(6750),6);
    }
    #[test]
    fn t_6750_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6750),2);
    }
    #[test]
    fn t_6750_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6750),-4);
    }
    #[test]
    fn t_6750_sen_test(){
        assert_eq!(test_sen_money_number(6750),4);
    }
    #[test]
    fn t_6750_gosen_test(){
        assert_eq!(test_gosen_money_number(6750),-6);
    }
    #[test]
    fn t_6750_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6750),-4);
    }
    // 6760
    #[test]
    fn t_6760_ju_test(){
        assert_eq!(test_ju_money_number(6760),30);
    }
    #[test]
    fn t_6760_goju_test(){
        assert_eq!(test_goju_money_number(6760),-2);
    }
    #[test]
    fn t_6760_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6760),2);
    }
    #[test]
    fn t_6760_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6760),-4);
    }
    #[test]
    fn t_6760_sen_test(){
        assert_eq!(test_sen_money_number(6760),4);
    }
    #[test]
    fn t_6760_gosen_test(){
        assert_eq!(test_gosen_money_number(6760),-6);
    }
    #[test]
    fn t_6760_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6760),-4);
    }
    // 6770
    #[test]
    fn t_6770_ju_test(){
        assert_eq!(test_ju_money_number(6770),20);
    }
    #[test]
    fn t_6770_goju_test(){
        assert_eq!(test_goju_money_number(6770),-2);
    }
    #[test]
    fn t_6770_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6770),2);
    }
    #[test]
    fn t_6770_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6770),-4);
    }
    #[test]
    fn t_6770_sen_test(){
        assert_eq!(test_sen_money_number(6770),4);
    }
    #[test]
    fn t_6770_gosen_test(){
        assert_eq!(test_gosen_money_number(6770),-6);
    }
    #[test]
    fn t_6770_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6770),-4);
    }
    // 6780
    #[test]
    fn t_6780_ju_test(){
        assert_eq!(test_ju_money_number(6780),10);
    }
    #[test]
    fn t_6780_goju_test(){
        assert_eq!(test_goju_money_number(6780),-2);
    }
    #[test]
    fn t_6780_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6780),2);
    }
    #[test]
    fn t_6780_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6780),-4);
    }
    #[test]
    fn t_6780_sen_test(){
        assert_eq!(test_sen_money_number(6780),4);
    }
    #[test]
    fn t_6780_gosen_test(){
        assert_eq!(test_gosen_money_number(6780),-6);
    }
    #[test]
    fn t_6780_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6780),-4);
    }
    // 6790
    #[test]
    fn t_6790_ju_test(){
        assert_eq!(test_ju_money_number(6790),0);
    }
    #[test]
    fn t_6790_goju_test(){
        assert_eq!(test_goju_money_number(6790),-2);
    }
    #[test]
    fn t_6790_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6790),2);
    }
    #[test]
    fn t_6790_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6790),-4);
    }
    #[test]
    fn t_6790_sen_test(){
        assert_eq!(test_sen_money_number(6790),4);
    }
    #[test]
    fn t_6790_gosen_test(){
        assert_eq!(test_gosen_money_number(6790),-6);
    }
    #[test]
    fn t_6790_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6790),-4);
    }
    // 6800
    #[test]
    fn t_6800_ju_test(){
        assert_eq!(test_ju_money_number(6800),0);
    }
    #[test]
    fn t_6800_goju_test(){
        assert_eq!(test_goju_money_number(6800),0);
    }
    #[test]
    fn t_6800_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6800),5);
    }
    #[test]
    fn t_6800_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6800),-3);
    }
    #[test]
    fn t_6800_sen_test(){
        assert_eq!(test_sen_money_number(6800),3);
    }
    #[test]
    fn t_6800_gosen_test(){
        assert_eq!(test_gosen_money_number(6800),-6);
    }
    #[test]
    fn t_6800_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6800),-4);
    }
    // 6810
    #[test]
    fn t_6810_ju_test(){
        assert_eq!(test_ju_money_number(6810),30);
    }
    #[test]
    fn t_6810_goju_test(){
        assert_eq!(test_goju_money_number(6810),4);
    }
    #[test]
    fn t_6810_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6810),-16);
    }
    #[test]
    fn t_6810_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6810),-6);
    }
    #[test]
    fn t_6810_sen_test(){
        assert_eq!(test_sen_money_number(6810),-4);
    }
    #[test]
    fn t_6810_gosen_test(){
        assert_eq!(test_gosen_money_number(6810),-8);
    }
    #[test]
    fn t_6810_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6810),-2);
    }
    // 6820
    #[test]
    fn t_6820_ju_test(){
        assert_eq!(test_ju_money_number(6820),20);
    }
    #[test]
    fn t_6820_goju_test(){
        assert_eq!(test_goju_money_number(6820),4);
    }
    #[test]
    fn t_6820_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6820),-16);
    }
    #[test]
    fn t_6820_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6820),-6);
    }
    #[test]
    fn t_6820_sen_test(){
        assert_eq!(test_sen_money_number(6820),-4);
    }
    #[test]
    fn t_6820_gosen_test(){
        assert_eq!(test_gosen_money_number(6820),-8);
    }
    #[test]
    fn t_6820_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6820),-2);
    }
    // 6830
    #[test]
    fn t_6830_ju_test(){
        assert_eq!(test_ju_money_number(6830),10);
    }
    #[test]
    fn t_6830_goju_test(){
        assert_eq!(test_goju_money_number(6830),4);
    }
    #[test]
    fn t_6830_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6830),-16);
    }
    #[test]
    fn t_6830_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6830),-6);
    }
    #[test]
    fn t_6830_sen_test(){
        assert_eq!(test_sen_money_number(6830),-4);
    }
    #[test]
    fn t_6830_gosen_test(){
        assert_eq!(test_gosen_money_number(6830),-8);
    }
    #[test]
    fn t_6830_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6830),-2);
    }
    // 6840
    #[test]
    fn t_6840_ju_test(){
        assert_eq!(test_ju_money_number(6840),0);
    }
    #[test]
    fn t_6840_goju_test(){
        assert_eq!(test_goju_money_number(6840),4);
    }
    #[test]
    fn t_6840_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6840),-16);
    }
    #[test]
    fn t_6840_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6840),-6);
    }
    #[test]
    fn t_6840_sen_test(){
        assert_eq!(test_sen_money_number(6840),-4);
    }
    #[test]
    fn t_6840_gosen_test(){
        assert_eq!(test_gosen_money_number(6840),-8);
    }
    #[test]
    fn t_6840_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6840),-2);
    }
    // 6850
    #[test]
    fn t_6850_ju_test(){
        assert_eq!(test_ju_money_number(6850),0);
    }
    #[test]
    fn t_6850_goju_test(){
        assert_eq!(test_goju_money_number(6850),6);
    }
    #[test]
    fn t_6850_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6850),-8);
    }
    #[test]
    fn t_6850_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6850),-4);
    }
    #[test]
    fn t_6850_sen_test(){
        assert_eq!(test_sen_money_number(6850),4);
    }
    #[test]
    fn t_6850_gosen_test(){
        assert_eq!(test_gosen_money_number(6850),-6);
    }
    #[test]
    fn t_6850_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6850),-4);
    }
    // 6860
    #[test]
    fn t_6860_ju_test(){
        assert_eq!(test_ju_money_number(6860),30);
    }
    #[test]
    fn t_6860_goju_test(){
        assert_eq!(test_goju_money_number(6860),-2);
    }
    #[test]
    fn t_6860_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6860),-8);
    }
    #[test]
    fn t_6860_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6860),-4);
    }
    #[test]
    fn t_6860_sen_test(){
        assert_eq!(test_sen_money_number(6860),4);
    }
    #[test]
    fn t_6860_gosen_test(){
        assert_eq!(test_gosen_money_number(6860),-6);
    }
    #[test]
    fn t_6860_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6860),-4);
    }
    // 6870
    #[test]
    fn t_6870_ju_test(){
        assert_eq!(test_ju_money_number(6870),20);
    }
    #[test]
    fn t_6870_goju_test(){
        assert_eq!(test_goju_money_number(6870),-2);
    }
    #[test]
    fn t_6870_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6870),-8);
    }
    #[test]
    fn t_6870_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6870),-4);
    }
    #[test]
    fn t_6870_sen_test(){
        assert_eq!(test_sen_money_number(6870),4);
    }
    #[test]
    fn t_6870_gosen_test(){
        assert_eq!(test_gosen_money_number(6870),-6);
    }
    #[test]
    fn t_6870_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6870),-4);
    }
    // 6890
    #[test]
    fn t_6890_ju_test(){
        assert_eq!(test_ju_money_number(6890),0);
    }
    #[test]
    fn t_6890_goju_test(){
        assert_eq!(test_goju_money_number(6890),-2);
    }
    #[test]
    fn t_6890_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6890),-8);
    }
    #[test]
    fn t_6890_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6890),-4);
    }
    #[test]
    fn t_6890_sen_test(){
        assert_eq!(test_sen_money_number(6890),4);
    }
    #[test]
    fn t_6890_gosen_test(){
        assert_eq!(test_gosen_money_number(6890),-6);
    }
    #[test]
    fn t_6890_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6890),-4);
    }
    // 6900
    #[test]
    fn t_6900_ju_test(){
        assert_eq!(test_ju_money_number(6900),0);
    }
    #[test]
    fn t_6900_goju_test(){
        assert_eq!(test_goju_money_number(6900),0);
    }
    #[test]
    fn t_6900_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6900),-5);
    }
    #[test]
    fn t_6900_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6900),-3);
    }
    #[test]
    fn t_6900_sen_test(){
        assert_eq!(test_sen_money_number(6900),3);
    }
    #[test]
    fn t_6900_gosen_test(){
        assert_eq!(test_gosen_money_number(6900),-6);
    }
    #[test]
    fn t_6900_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6900),-4);
    }
    // 6910
    #[test]
    fn t_6910_ju_test(){
        assert_eq!(test_ju_money_number(6910),30);
    }
    #[test]
    fn t_6910_goju_test(){
        assert_eq!(test_goju_money_number(6910),4);
    }
    #[test]
    fn t_6910_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6910),-16);
    }
    #[test]
    fn t_6910_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6910),-4);
    }
    #[test]
    fn t_6910_sen_test(){
        assert_eq!(test_sen_money_number(6910),4);
    }
    #[test]
    fn t_6910_gosen_test(){
        assert_eq!(test_gosen_money_number(6910),-6);
    }
    #[test]
    fn t_6910_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6910),-4);
    }
    // 6920
    #[test]
    fn t_6920_ju_test(){
        assert_eq!(test_ju_money_number(6920),20);
    }
    #[test]
    fn t_6920_goju_test(){
        assert_eq!(test_goju_money_number(6920),4);
    }
    #[test]
    fn t_6920_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6920),-16);
    }
    #[test]
    fn t_6920_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6920),-4);
    }
    #[test]
    fn t_6920_sen_test(){
        assert_eq!(test_sen_money_number(6920),4);
    }
    #[test]
    fn t_6920_gosen_test(){
        assert_eq!(test_gosen_money_number(6920),-6);
    }
    #[test]
    fn t_6920_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6920),-4);
    }
    // 6930
    #[test]
    fn t_6930_ju_test(){
        assert_eq!(test_ju_money_number(6930),10);
    }
    #[test]
    fn t_6930_goju_test(){
        assert_eq!(test_goju_money_number(6930),4);
    }
    #[test]
    fn t_6930_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6930),-16);
    }
    #[test]
    fn t_6930_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6930),-4);
    }
    #[test]
    fn t_6930_sen_test(){
        assert_eq!(test_sen_money_number(6930),4);
    }
    #[test]
    fn t_6930_gosen_test(){
        assert_eq!(test_gosen_money_number(6930),-6);
    }
    #[test]
    fn t_6930_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6930),-4);
    }
    // 6940
    #[test]
    fn t_6940_ju_test(){
        assert_eq!(test_ju_money_number(6940),0);
    }
    #[test]
    fn t_6940_goju_test(){
        assert_eq!(test_goju_money_number(6940),4);
    }
    #[test]
    fn t_6940_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6940),-16);
    }
    #[test]
    fn t_6940_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6940),-4);
    }
    #[test]
    fn t_6940_sen_test(){
        assert_eq!(test_sen_money_number(6940),4);
    }
    #[test]
    fn t_6940_gosen_test(){
        assert_eq!(test_gosen_money_number(6940),-6);
    }
    #[test]
    fn t_6940_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6940),-4);
    }
    // 6950
    #[test]
    fn t_6950_ju_test(){
        assert_eq!(test_ju_money_number(6950),0);
    }
    #[test]
    fn t_6950_goju_test(){
        assert_eq!(test_goju_money_number(6950),4);
    }
    #[test]
    fn t_6950_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6950),-12);
    }
    #[test]
    fn t_6950_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6950),-3);
    }
    #[test]
    fn t_6950_sen_test(){
        assert_eq!(test_sen_money_number(6950),3);
    }
    #[test]
    fn t_6950_gosen_test(){
        assert_eq!(test_gosen_money_number(6950),-6);
    }
    #[test]
    fn t_6950_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6950),-4);
    }
    // 6960
    #[test]
    fn t_6960_ju_test(){
        assert_eq!(test_ju_money_number(6960),25);
    }
    #[test]
    fn t_6960_goju_test(){
        assert_eq!(test_goju_money_number(6960),-3);
    }
    #[test]
    fn t_6960_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6960),-12);
    }
    #[test]
    fn t_6960_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6960),-3);
    }
    #[test]
    fn t_6960_sen_test(){
        assert_eq!(test_sen_money_number(6960),3);
    }
    #[test]
    fn t_6960_gosen_test(){
        assert_eq!(test_gosen_money_number(6960),-6);
    }
    #[test]
    fn t_6960_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6960),-4);
    }
    // 6970
    #[test]
    fn t_6970_ju_test(){
        assert_eq!(test_ju_money_number(6970),15);
    }
    #[test]
    fn t_6970_goju_test(){
        assert_eq!(test_goju_money_number(6970),-3);
    }
    #[test]
    fn t_6970_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6970),-12);
    }
    #[test]
    fn t_6970_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6970),-3);
    }
    #[test]
    fn t_6970_sen_test(){
        assert_eq!(test_sen_money_number(6970),3);
    }
    #[test]
    fn t_6970_gosen_test(){
        assert_eq!(test_gosen_money_number(6970),-6);
    }
    #[test]
    fn t_6970_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6970),-4);
    }
    // 6980
    #[test]
    fn t_6980_ju_test(){
        assert_eq!(test_ju_money_number(6980),5);
    }
    #[test]
    fn t_6980_goju_test(){
        assert_eq!(test_goju_money_number(6980),-3);
    }
    #[test]
    fn t_6980_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6980),-12);
    }
    #[test]
    fn t_6980_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6980),-3);
    }
    #[test]
    fn t_6980_sen_test(){
        assert_eq!(test_sen_money_number(6980),3);
    }
    #[test]
    fn t_6980_gosen_test(){
        assert_eq!(test_gosen_money_number(6980),-6);
    }
    #[test]
    fn t_6980_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6980),-4);
    }
    // 6990
    #[test]
    fn t_6990_ju_test(){
        assert_eq!(test_ju_money_number(6990),-5);
    }
    #[test]
    fn t_6990_goju_test(){
        assert_eq!(test_goju_money_number(6990),-3);
    }
    #[test]
    fn t_6990_hyaku_test(){
        assert_eq!(test_hyaku_money_number(6990),-12);
    }
    #[test]
    fn t_6990_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(6990),-3);
    }
    #[test]
    fn t_6990_sen_test(){
        assert_eq!(test_sen_money_number(6990),3);
    }
    #[test]
    fn t_6990_gosen_test(){
        assert_eq!(test_gosen_money_number(6990),-6);
    }
    #[test]
    fn t_6990_ichiman_test(){
        assert_eq!(test_ichiman_money_number(6990),-4);
    }
    // 7000
    #[test]
    fn t_7000_ju_test(){
        assert_eq!(test_ju_money_number(7000),0);
    }
    #[test]
    fn t_7000_goju_test(){
        assert_eq!(test_goju_money_number(7000),0);
    }
    #[test]
    fn t_7000_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7000),0);
    }
    #[test]
    fn t_7000_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7000),0);
    }
    #[test]
    fn t_7000_sen_test(){
        assert_eq!(test_sen_money_number(7000),5);
    }
    #[test]
    fn t_7000_gosen_test(){
        assert_eq!(test_gosen_money_number(7000),-5);
    }
    #[test]
    fn t_7000_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7000),-5);
    }
    // 7010
    #[test]
    fn t_7010_ju_test(){
        assert_eq!(test_ju_money_number(7010),35);
    }
    #[test]
    fn t_7010_goju_test(){
        assert_eq!(test_goju_money_number(7010),7);
    }
    #[test]
    fn t_7010_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7010),27);
    }
    #[test]
    fn t_7010_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7010),5);
    }
    #[test]
    fn t_7010_sen_test(){
        assert_eq!(test_sen_money_number(7010),-1);
    }
    #[test]
    fn t_7010_gosen_test(){
        assert_eq!(test_gosen_money_number(7010),-5);
    }
    #[test]
    fn t_7010_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7010),-5);
    }
    // 7020
    #[test]
    fn t_7020_ju_test(){
        assert_eq!(test_ju_money_number(7020),25);
    }
    #[test]
    fn t_7020_goju_test(){
        assert_eq!(test_goju_money_number(7020),7);
    }
    #[test]
    fn t_7020_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7020),27);
    }
    #[test]
    fn t_7020_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7020),5);
    }
    #[test]
    fn t_7020_sen_test(){
        assert_eq!(test_sen_money_number(7020),-1);
    }
    #[test]
    fn t_7020_gosen_test(){
        assert_eq!(test_gosen_money_number(7020),-5);
    }
    #[test]
    fn t_7020_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7020),-5);
    }
    // 7030
    #[test]
    fn t_7030_ju_test(){
        assert_eq!(test_ju_money_number(7030),15);
    }
    #[test]
    fn t_7030_goju_test(){
        assert_eq!(test_goju_money_number(7030),7);
    }
    #[test]
    fn t_7030_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7030),27);
    }
    #[test]
    fn t_7030_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7030),5);
    }
    #[test]
    fn t_7030_sen_test(){
        assert_eq!(test_sen_money_number(7030),-1);
    }
    #[test]
    fn t_7030_gosen_test(){
        assert_eq!(test_gosen_money_number(7030),-5);
    }
    #[test]
    fn t_7030_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7030),-5);
    }
    // 7040
    #[test]
    fn t_7040_ju_test(){
        assert_eq!(test_ju_money_number(7040),5);
    }
    #[test]
    fn t_7040_goju_test(){
        assert_eq!(test_goju_money_number(7040),7);
    }
    #[test]
    fn t_7040_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7040),27);
    }
    #[test]
    fn t_7040_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7040),5);
    }
    #[test]
    fn t_7040_sen_test(){
        assert_eq!(test_sen_money_number(7040),-1);
    }
    #[test]
    fn t_7040_gosen_test(){
        assert_eq!(test_gosen_money_number(7040),-5);
    }
    #[test]
    fn t_7040_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7040),-5);
    }
    // 7050
    #[test]
    fn t_7050_ju_test(){
        assert_eq!(test_ju_money_number(7050),0);
    }
    #[test]
    fn t_7050_goju_test(){
        assert_eq!(test_goju_money_number(7050),6);
    }
    #[test]
    fn t_7050_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7050),22);
    }
    #[test]
    fn t_7050_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7050),2);
    }
    #[test]
    fn t_7050_sen_test(){
        assert_eq!(test_sen_money_number(7050),-14);
    }
    #[test]
    fn t_7050_gosen_test(){
        assert_eq!(test_gosen_money_number(7050),-8);
    }
    #[test]
    fn t_7050_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7050),-2);
    }
    // 7060
    #[test]
    fn t_7060_ju_test(){
        assert_eq!(test_ju_money_number(7060),30);
    }
    #[test]
    fn t_7060_goju_test(){
        assert_eq!(test_goju_money_number(7060),-2);
    }
    #[test]
    fn t_7060_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7060),22);
    }
    #[test]
    fn t_7060_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7060),2);
    }
    #[test]
    fn t_7060_sen_test(){
        assert_eq!(test_sen_money_number(7060),-14);
    }
    #[test]
    fn t_7060_gosen_test(){
        assert_eq!(test_gosen_money_number(7060),-8);
    }
    #[test]
    fn t_7060_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7060),-2);
    }
    // 7070
    #[test]
    fn t_7070_ju_test(){
        assert_eq!(test_ju_money_number(7070),20);
    }
    #[test]
    fn t_7070_goju_test(){
        assert_eq!(test_goju_money_number(7070),-2);
    }
    #[test]
    fn t_7070_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7070),22);
    }
    #[test]
    fn t_7070_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7070),2);
    }
    #[test]
    fn t_7070_sen_test(){
        assert_eq!(test_sen_money_number(7070),-14);
    }
    #[test]
    fn t_7070_gosen_test(){
        assert_eq!(test_gosen_money_number(7070),-8);
    }
    #[test]
    fn t_7070_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7070),-2);
    }
    // 7080
    #[test]
    fn t_7080_ju_test(){
        assert_eq!(test_ju_money_number(7080),10);
    }
    #[test]
    fn t_7080_goju_test(){
        assert_eq!(test_goju_money_number(7080),-2);
    }
    #[test]
    fn t_7080_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7080),22);
    }
    #[test]
    fn t_7080_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7080),2);
    }
    #[test]
    fn t_7080_sen_test(){
        assert_eq!(test_sen_money_number(7080),-14);
    }
    #[test]
    fn t_7080_gosen_test(){
        assert_eq!(test_gosen_money_number(7080),-8);
    }
    #[test]
    fn t_7080_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7080),-2);
    }
    // 7090
    #[test]
    fn t_7090_ju_test(){
        assert_eq!(test_ju_money_number(7090),0);
    }
    #[test]
    fn t_7090_goju_test(){
        assert_eq!(test_goju_money_number(7090),-2);
    }
    #[test]
    fn t_7090_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7090),22);
    }
    #[test]
    fn t_7090_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7090),2);
    }
    #[test]
    fn t_7090_sen_test(){
        assert_eq!(test_sen_money_number(7090),-14);
    }
    #[test]
    fn t_7090_gosen_test(){
        assert_eq!(test_gosen_money_number(7090),-8);
    }
    #[test]
    fn t_7090_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7090),-2);
    }
    // 7100
    #[test]
    fn t_7100_ju_test(){
        assert_eq!(test_ju_money_number(7100),0);
    }
    #[test]
    fn t_7100_goju_test(){
        assert_eq!(test_goju_money_number(7100),0);
    }
    #[test]
    fn t_7100_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7100),30);
    }
    #[test]
    fn t_7100_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7100),4);
    }
    #[test]
    fn t_7100_sen_test(){
        assert_eq!(test_sen_money_number(7100),-6);
    }
    #[test]
    fn t_7100_gosen_test(){
        assert_eq!(test_gosen_money_number(7100),-6);
    }
    #[test]
    fn t_7100_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7100),-4);
    }
    // 7110
    #[test]
    fn t_7110_ju_test(){
        assert_eq!(test_ju_money_number(7110),35);
    }
    #[test]
    fn t_7110_goju_test(){
        assert_eq!(test_goju_money_number(7110),7);
    }
    #[test]
    fn t_7110_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7110),17);
    }
    #[test]
    fn t_7110_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7110),5);
    }
    #[test]
    fn t_7110_sen_test(){
        assert_eq!(test_sen_money_number(7110),-1);
    }
    #[test]
    fn t_7110_gosen_test(){
        assert_eq!(test_gosen_money_number(7110),-5);
    }
    #[test]
    fn t_7110_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7110),-5);
    }
    // 7120
    #[test]
    fn t_7120_ju_test(){
        assert_eq!(test_ju_money_number(7120),25);
    }
    #[test]
    fn t_7120_goju_test(){
        assert_eq!(test_goju_money_number(7120),7);
    }
    #[test]
    fn t_7120_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7120),17);
    }
    #[test]
    fn t_7120_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7120),5);
    }
    #[test]
    fn t_7120_sen_test(){
        assert_eq!(test_sen_money_number(7120),-1);
    }
    #[test]
    fn t_7120_gosen_test(){
        assert_eq!(test_gosen_money_number(7120),-5);
    }
    #[test]
    fn t_7120_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7120),-5);
    }
    // 7130
    #[test]
    fn t_7130_ju_test(){
        assert_eq!(test_ju_money_number(7130),15);
    }
    #[test]
    fn t_7130_goju_test(){
        assert_eq!(test_goju_money_number(7130),7);
    }
    #[test]
    fn t_7130_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7130),17);
    }
    #[test]
    fn t_7130_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7130),5);
    }
    #[test]
    fn t_7130_sen_test(){
        assert_eq!(test_sen_money_number(7130),-1);
    }
    #[test]
    fn t_7130_gosen_test(){
        assert_eq!(test_gosen_money_number(7130),-5);
    }
    #[test]
    fn t_7130_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7130),-5);
    }
    // 7140
    #[test]
    fn t_7140_ju_test(){
        assert_eq!(test_ju_money_number(7140),5);
    }
    #[test]
    fn t_7140_goju_test(){
        assert_eq!(test_goju_money_number(7140),7);
    }
    #[test]
    fn t_7140_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7140),17);
    }
    #[test]
    fn t_7140_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7140),5);
    }
    #[test]
    fn t_7140_sen_test(){
        assert_eq!(test_sen_money_number(7140),-1);
    }
    #[test]
    fn t_7140_gosen_test(){
        assert_eq!(test_gosen_money_number(7140),-5);
    }
    #[test]
    fn t_7140_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7140),-5);
    }
    // 7150
    #[test]
    fn t_7150_ju_test(){
        assert_eq!(test_ju_money_number(7150),0);
    }
    #[test]
    fn t_7150_goju_test(){
        assert_eq!(test_goju_money_number(7150),6);
    }
    #[test]
    fn t_7150_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7150),12);
    }
    #[test]
    fn t_7150_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7150),2);
    }
    #[test]
    fn t_7150_sen_test(){
        assert_eq!(test_sen_money_number(7150),-14);
    }
    #[test]
    fn t_7150_gosen_test(){
        assert_eq!(test_gosen_money_number(7150),-8);
    }
    #[test]
    fn t_7150_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7150),-2);
    }
    // 7160
    #[test]
    fn t_7160_ju_test(){
        assert_eq!(test_ju_money_number(7160),30);
    }
    #[test]
    fn t_7160_goju_test(){
        assert_eq!(test_goju_money_number(7160),-2);
    }
    #[test]
    fn t_7160_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7160),12);
    }
    #[test]
    fn t_7160_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7160),2);
    }
    #[test]
    fn t_7160_sen_test(){
        assert_eq!(test_sen_money_number(7160),-14);
    }
    #[test]
    fn t_7160_gosen_test(){
        assert_eq!(test_gosen_money_number(7160),-8);
    }
    #[test]
    fn t_7160_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7160),-2);
    }
    // 7170
    #[test]
    fn t_7170_ju_test(){
        assert_eq!(test_ju_money_number(7170),20);
    }
    #[test]
    fn t_7170_goju_test(){
        assert_eq!(test_goju_money_number(7170),-2);
    }
    #[test]
    fn t_7170_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7170),12);
    }
    #[test]
    fn t_7170_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7170),2);
    }
    #[test]
    fn t_7170_sen_test(){
        assert_eq!(test_sen_money_number(7170),-14);
    }
    #[test]
    fn t_7170_gosen_test(){
        assert_eq!(test_gosen_money_number(7170),-8);
    }
    #[test]
    fn t_7170_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7170),-2);
    }
    // 7180
    #[test]
    fn t_7180_ju_test(){
        assert_eq!(test_ju_money_number(7180),10);
    }
    #[test]
    fn t_7180_goju_test(){
        assert_eq!(test_goju_money_number(7180),-2);
    }
    #[test]
    fn t_7180_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7180),12);
    }
    #[test]
    fn t_7180_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7180),2);
    }
    #[test]
    fn t_7180_sen_test(){
        assert_eq!(test_sen_money_number(7180),-14);
    }
    #[test]
    fn t_7180_gosen_test(){
        assert_eq!(test_gosen_money_number(7180),-8);
    }
    #[test]
    fn t_7180_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7180),-2);
    }
    // 7190
    #[test]
    fn t_7190_ju_test(){
        assert_eq!(test_ju_money_number(7190),0);
    }
    #[test]
    fn t_7190_goju_test(){
        assert_eq!(test_goju_money_number(7190),-2);
    }
    #[test]
    fn t_7190_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7190),12);
    }
    #[test]
    fn t_7190_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7190),2);
    }
    #[test]
    fn t_7190_sen_test(){
        assert_eq!(test_sen_money_number(7190),-14);
    }
    #[test]
    fn t_7190_gosen_test(){
        assert_eq!(test_gosen_money_number(7190),-8);
    }
    #[test]
    fn t_7190_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7190),-2);
    }
    // 7200
    #[test]
    fn t_7200_ju_test(){
        assert_eq!(test_ju_money_number(7200),0);
    }
    #[test]
    fn t_7200_goju_test(){
        assert_eq!(test_goju_money_number(7200),0);
    }
    #[test]
    fn t_7200_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7200),20);
    }
    #[test]
    fn t_7200_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7200),4);
    }
    #[test]
    fn t_7200_sen_test(){
        assert_eq!(test_sen_money_number(7200),-6);
    }
    #[test]
    fn t_7200_gosen_test(){
        assert_eq!(test_gosen_money_number(7200),-6);
    }
    #[test]
    fn t_7200_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7200),-4);
    }
    // 7210
    #[test]
    fn t_7210_ju_test(){
        assert_eq!(test_ju_money_number(7210),35);
    }
    #[test]
    fn t_7210_goju_test(){
        assert_eq!(test_goju_money_number(7210),7);
    }
    #[test]
    fn t_7210_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7210),7);
    }
    #[test]
    fn t_7210_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7210),5);
    }
    #[test]
    fn t_7210_sen_test(){
        assert_eq!(test_sen_money_number(7210),-1);
    }
    #[test]
    fn t_7210_gosen_test(){
        assert_eq!(test_gosen_money_number(7210),-5);
    }
    #[test]
    fn t_7210_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7210),-5);
    }
    // 7220
    #[test]
    fn t_7220_ju_test(){
        assert_eq!(test_ju_money_number(7220),25);
    }
    #[test]
    fn t_7220_goju_test(){
        assert_eq!(test_goju_money_number(7220),7);
    }
    #[test]
    fn t_7220_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7220),7);
    }
    #[test]
    fn t_7220_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7220),5);
    }
    #[test]
    fn t_7220_sen_test(){
        assert_eq!(test_sen_money_number(7220),-1);
    }
    #[test]
    fn t_7220_gosen_test(){
        assert_eq!(test_gosen_money_number(7220),-5);
    }
    #[test]
    fn t_7220_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7220),-5);
    }
    // 7230
    #[test]
    fn t_7230_ju_test(){
        assert_eq!(test_ju_money_number(7230),15);
    }
    #[test]
    fn t_7230_goju_test(){
        assert_eq!(test_goju_money_number(7230),7);
    }
    #[test]
    fn t_7230_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7230),7);
    }
    #[test]
    fn t_7230_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7230),5);
    }
    #[test]
    fn t_7230_sen_test(){
        assert_eq!(test_sen_money_number(7230),-1);
    }
    #[test]
    fn t_7230_gosen_test(){
        assert_eq!(test_gosen_money_number(7230),-5);
    }
    #[test]
    fn t_7230_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7230),-5);
    }
    // 7240
    #[test]
    fn t_7240_ju_test(){
        assert_eq!(test_ju_money_number(7240),5);
    }
    #[test]
    fn t_7240_goju_test(){
        assert_eq!(test_goju_money_number(7240),7);
    }
    #[test]
    fn t_7240_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7240),7);
    }
    #[test]
    fn t_7240_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7240),5);
    }
    #[test]
    fn t_7240_sen_test(){
        assert_eq!(test_sen_money_number(7240),-1);
    }
    #[test]
    fn t_7240_gosen_test(){
        assert_eq!(test_gosen_money_number(7240),-5);
    }
    #[test]
    fn t_7240_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7240),-5);
    }
    // 7250
    #[test]
    fn t_7250_ju_test(){
        assert_eq!(test_ju_money_number(7250),0);
    }
    #[test]
    fn t_7250_goju_test(){
        assert_eq!(test_goju_money_number(7250),6);
    }
    #[test]
    fn t_7250_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7250),2);
    }
    #[test]
    fn t_7250_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7250),2);
    }
    #[test]
    fn t_7250_sen_test(){
        assert_eq!(test_sen_money_number(7250),-14);
    }
    #[test]
    fn t_7250_gosen_test(){
        assert_eq!(test_gosen_money_number(7250),-8);
    }
    #[test]
    fn t_7250_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7250),-2);
    }
    // 7260
    #[test]
    fn t_7260_ju_test(){
        assert_eq!(test_ju_money_number(7260),30);
    }
    #[test]
    fn t_7260_goju_test(){
        assert_eq!(test_goju_money_number(7260),-2);
    }
    #[test]
    fn t_7260_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7260),2);
    }
    #[test]
    fn t_7260_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7260),2);
    }
    #[test]
    fn t_7260_sen_test(){
        assert_eq!(test_sen_money_number(7260),-14);
    }
    #[test]
    fn t_7260_gosen_test(){
        assert_eq!(test_gosen_money_number(7260),-8);
    }
    #[test]
    fn t_7260_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7260),-2);
    }
    // 7270
    #[test]
    fn t_7270_ju_test(){
        assert_eq!(test_ju_money_number(7270),20);
    }
    #[test]
    fn t_7270_goju_test(){
        assert_eq!(test_goju_money_number(7270),-2);
    }
    #[test]
    fn t_7270_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7270),2);
    }
    #[test]
    fn t_7270_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7270),2);
    }
    #[test]
    fn t_7270_sen_test(){
        assert_eq!(test_sen_money_number(7270),-14);
    }
    #[test]
    fn t_7270_gosen_test(){
        assert_eq!(test_gosen_money_number(7270),-8);
    }
    #[test]
    fn t_7270_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7270),-2);
    }
    // 7280
    #[test]
    fn t_7280_ju_test(){
        assert_eq!(test_ju_money_number(7280),10);
    }
    #[test]
    fn t_7280_goju_test(){
        assert_eq!(test_goju_money_number(7280),-2);
    }
    #[test]
    fn t_7280_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7280),2);
    }
    #[test]
    fn t_7280_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7280),2);
    }
    #[test]
    fn t_7280_sen_test(){
        assert_eq!(test_sen_money_number(7280),-14);
    }
    #[test]
    fn t_7280_gosen_test(){
        assert_eq!(test_gosen_money_number(7280),-8);
    }
    #[test]
    fn t_7280_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7280),-2);
    }
    // 7290
    #[test]
    fn t_7290_ju_test(){
        assert_eq!(test_ju_money_number(7290),0);
    }
    #[test]
    fn t_7290_goju_test(){
        assert_eq!(test_goju_money_number(7290),-2);
    }
    #[test]
    fn t_7290_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7290),2);
    }
    #[test]
    fn t_7290_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7290),2);
    }
    #[test]
    fn t_7290_sen_test(){
        assert_eq!(test_sen_money_number(7290),-14);
    }
    #[test]
    fn t_7290_gosen_test(){
        assert_eq!(test_gosen_money_number(7290),-8);
    }
    #[test]
    fn t_7290_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7290),-2);
    }
    // 7300
    #[test]
    fn t_7300_ju_test(){
        assert_eq!(test_ju_money_number(7300),0);
    }
    #[test]
    fn t_7300_goju_test(){
        assert_eq!(test_goju_money_number(7300),0);
    }
    #[test]
    fn t_7300_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7300),10);
    }
    #[test]
    fn t_7300_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7300),4);
    }
    #[test]
    fn t_7300_sen_test(){
        assert_eq!(test_sen_money_number(7300),-6);
    }
    #[test]
    fn t_7300_gosen_test(){
        assert_eq!(test_gosen_money_number(7300),-6);
    }
    #[test]
    fn t_7300_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7300),-4);
    }
    // 7310
    #[test]
    fn t_7310_ju_test(){
        assert_eq!(test_ju_money_number(7310),35);
    }
    #[test]
    fn t_7310_goju_test(){
        assert_eq!(test_goju_money_number(7310),7);
    }
    #[test]
    fn t_7310_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7310),-3);
    }
    #[test]
    fn t_7310_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7310),5);
    }
    #[test]
    fn t_7310_sen_test(){
        assert_eq!(test_sen_money_number(7310),-1);
    }
    #[test]
    fn t_7310_gosen_test(){
        assert_eq!(test_gosen_money_number(7310),-5);
    }
    #[test]
    fn t_7310_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7310),-5);
    }
    // 7320
    #[test]
    fn t_7320_ju_test(){
        assert_eq!(test_ju_money_number(7320),25);
    }
    #[test]
    fn t_7320_goju_test(){
        assert_eq!(test_goju_money_number(7320),7);
    }
    #[test]
    fn t_7320_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7320),-3);
    }
    #[test]
    fn t_7320_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7320),5);
    }
    #[test]
    fn t_7320_sen_test(){
        assert_eq!(test_sen_money_number(7320),-1);
    }
    #[test]
    fn t_7320_gosen_test(){
        assert_eq!(test_gosen_money_number(7320),-5);
    }
    #[test]
    fn t_7320_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7320),-5);
    }
    // 7330
    #[test]
    fn t_7330_ju_test(){
        assert_eq!(test_ju_money_number(7330),15);
    }
    #[test]
    fn t_7330_goju_test(){
        assert_eq!(test_goju_money_number(7330),7);
    }
    #[test]
    fn t_7330_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7330),-3);
    }
    #[test]
    fn t_7330_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7330),5);
    }
    #[test]
    fn t_7330_sen_test(){
        assert_eq!(test_sen_money_number(7330),-1);
    }
    #[test]
    fn t_7330_gosen_test(){
        assert_eq!(test_gosen_money_number(7330),-5);
    }
    #[test]
    fn t_7330_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7330),-5);
    }
    // 7340
    #[test]
    fn t_7340_ju_test(){
        assert_eq!(test_ju_money_number(7340),5);
    }
    #[test]
    fn t_7340_goju_test(){
        assert_eq!(test_goju_money_number(7340),7);
    }
    #[test]
    fn t_7340_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7340),-3);
    }
    #[test]
    fn t_7340_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7340),5);
    }
    #[test]
    fn t_7340_sen_test(){
        assert_eq!(test_sen_money_number(7340),-1);
    }
    #[test]
    fn t_7340_gosen_test(){
        assert_eq!(test_gosen_money_number(7340),-5);
    }
    #[test]
    fn t_7340_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7340),-5);
    }
    // 7350
    #[test]
    fn t_7350_ju_test(){
        assert_eq!(test_ju_money_number(7350),0);
    }
    #[test]
    fn t_7350_goju_test(){
        assert_eq!(test_goju_money_number(7350),6);
    }
    #[test]
    fn t_7350_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7350),-8);
    }
    #[test]
    fn t_7350_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7350),2);
    }
    #[test]
    fn t_7350_sen_test(){
        assert_eq!(test_sen_money_number(7350),-14);
    }
    #[test]
    fn t_7350_gosen_test(){
        assert_eq!(test_gosen_money_number(7350),-8);
    }
    #[test]
    fn t_7350_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7350),-2);
    }
    // 7360
    #[test]
    fn t_7360_ju_test(){
        assert_eq!(test_ju_money_number(7360),30);
    }
    #[test]
    fn t_7360_goju_test(){
        assert_eq!(test_goju_money_number(7360),-2);
    }
    #[test]
    fn t_7360_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7360),-8);
    }
    #[test]
    fn t_7360_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7360),2);
    }
    #[test]
    fn t_7360_sen_test(){
        assert_eq!(test_sen_money_number(7360),-14);
    }
    #[test]
    fn t_7360_gosen_test(){
        assert_eq!(test_gosen_money_number(7360),-8);
    }
    #[test]
    fn t_7360_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7360),-2);
    }
    // 7370
    #[test]
    fn t_7370_ju_test(){
        assert_eq!(test_ju_money_number(7370),20);
    }
    #[test]
    fn t_7370_goju_test(){
        assert_eq!(test_goju_money_number(7370),-2);
    }
    #[test]
    fn t_7370_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7370),-8);
    }
    #[test]
    fn t_7370_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7370),2);
    }
    #[test]
    fn t_7370_sen_test(){
        assert_eq!(test_sen_money_number(7370),-14);
    }
    #[test]
    fn t_7370_gosen_test(){
        assert_eq!(test_gosen_money_number(7370),-8);
    }
    #[test]
    fn t_7370_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7370),-2);
    }
    // 7380
    #[test]
    fn t_7380_ju_test(){
        assert_eq!(test_ju_money_number(7380),10);
    }
    #[test]
    fn t_7380_goju_test(){
        assert_eq!(test_goju_money_number(7380),-2);
    }
    #[test]
    fn t_7380_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7380),-8);
    }
    #[test]
    fn t_7380_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7380),2);
    }
    #[test]
    fn t_7380_sen_test(){
        assert_eq!(test_sen_money_number(7380),-14);
    }
    #[test]
    fn t_7380_gosen_test(){
        assert_eq!(test_gosen_money_number(7380),-8);
    }
    #[test]
    fn t_7380_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7380),-2);
    }
    // 7390
    #[test]
    fn t_7390_ju_test(){
        assert_eq!(test_ju_money_number(7390),0);
    }
    #[test]
    fn t_7390_goju_test(){
        assert_eq!(test_goju_money_number(7390),-2);
    }
    #[test]
    fn t_7390_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7390),-8);
    }
    #[test]
    fn t_7390_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7390),2);
    }
    #[test]
    fn t_7390_sen_test(){
        assert_eq!(test_sen_money_number(7390),-14);
    }
    #[test]
    fn t_7390_gosen_test(){
        assert_eq!(test_gosen_money_number(7390),-8);
    }
    #[test]
    fn t_7390_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7390),-2);
    }
    // 7400
    #[test]
    fn t_7400_ju_test(){
        assert_eq!(test_ju_money_number(7400),0);
    }
    #[test]
    fn t_7400_goju_test(){
        assert_eq!(test_goju_money_number(7400),0);
    }
    #[test]
    fn t_7400_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7400),0);
    }
    #[test]
    fn t_7400_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7400),4);
    }
    #[test]
    fn t_7400_sen_test(){
        assert_eq!(test_sen_money_number(7400),-6);
    }
    #[test]
    fn t_7400_gosen_test(){
        assert_eq!(test_gosen_money_number(7400),-6);
    }
    #[test]
    fn t_7400_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7400),-4);
    }
    // 7410
    #[test]
    fn t_7410_ju_test(){
        assert_eq!(test_ju_money_number(7410),30);
    }
    #[test]
    fn t_7410_goju_test(){
        assert_eq!(test_goju_money_number(7410),4);
    }
    #[test]
    fn t_7410_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7410),-16);
    }
    #[test]
    fn t_7410_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7410),2);
    }
    #[test]
    fn t_7410_sen_test(){
        assert_eq!(test_sen_money_number(7410),-14);
    }
    #[test]
    fn t_7410_gosen_test(){
        assert_eq!(test_gosen_money_number(7410),-8);
    }
    #[test]
    fn t_7410_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7410),-2);
    }
    // 7420
    #[test]
    fn t_7420_ju_test(){
        assert_eq!(test_ju_money_number(7420),20);
    }
    #[test]
    fn t_7420_goju_test(){
        assert_eq!(test_goju_money_number(7420),4);
    }
    #[test]
    fn t_7420_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7420),-16);
    }
    #[test]
    fn t_7420_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7420),2);
    }
    #[test]
    fn t_7420_sen_test(){
        assert_eq!(test_sen_money_number(7420),-14);
    }
    #[test]
    fn t_7420_gosen_test(){
        assert_eq!(test_gosen_money_number(7420),-8);
    }
    #[test]
    fn t_7420_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7420),-2);
    }
    // 7430
    #[test]
    fn t_7430_ju_test(){
        assert_eq!(test_ju_money_number(7430),10);
    }
    #[test]
    fn t_7430_goju_test(){
        assert_eq!(test_goju_money_number(7430),4);
    }
    #[test]
    fn t_7430_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7430),-16);
    }
    #[test]
    fn t_7430_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7430),2);
    }
    #[test]
    fn t_7430_sen_test(){
        assert_eq!(test_sen_money_number(7430),-14);
    }
    #[test]
    fn t_7430_gosen_test(){
        assert_eq!(test_gosen_money_number(7430),-8);
    }
    #[test]
    fn t_7430_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7430),-2);
    }
    // 7440
    #[test]
    fn t_7440_ju_test(){
        assert_eq!(test_ju_money_number(7440),0);
    }
    #[test]
    fn t_7440_goju_test(){
        assert_eq!(test_goju_money_number(7440),4);
    }
    #[test]
    fn t_7440_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7440),-16);
    }
    #[test]
    fn t_7440_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7440),2);
    }
    #[test]
    fn t_7440_sen_test(){
        assert_eq!(test_sen_money_number(7440),-14);
    }
    #[test]
    fn t_7440_gosen_test(){
        assert_eq!(test_gosen_money_number(7440),-8);
    }
    #[test]
    fn t_7440_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7440),-2);
    }
    // 7450
    #[test]
    fn t_7450_ju_test(){
        assert_eq!(test_ju_money_number(7450),0);
    }
    #[test]
    fn t_7450_goju_test(){
        assert_eq!(test_goju_money_number(7450),6);
    }
    #[test]
    fn t_7450_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7450),-8);
    }
    #[test]
    fn t_7450_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7450),4);
    }
    #[test]
    fn t_7450_sen_test(){
        assert_eq!(test_sen_money_number(7450),-6);
    }
    #[test]
    fn t_7450_gosen_test(){
        assert_eq!(test_gosen_money_number(7450),-6);
    }
    #[test]
    fn t_7450_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7450),-4);
    }
    // 7460
    #[test]
    fn t_7460_ju_test(){
        assert_eq!(test_ju_money_number(7460),30);
    }
    #[test]
    fn t_7460_goju_test(){
        assert_eq!(test_goju_money_number(7460),-2);
    }
    #[test]
    fn t_7460_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7460),-8);
    }
    #[test]
    fn t_7460_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7460),4);
    }
    #[test]
    fn t_7460_sen_test(){
        assert_eq!(test_sen_money_number(7460),-6);
    }
    #[test]
    fn t_7460_gosen_test(){
        assert_eq!(test_gosen_money_number(7460),-6);
    }
    #[test]
    fn t_7460_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7460),-4);
    }
    // 7470
    #[test]
    fn t_7470_ju_test(){
        assert_eq!(test_ju_money_number(7470),20);
    }
    #[test]
    fn t_7470_goju_test(){
        assert_eq!(test_goju_money_number(7470),-2);
    }
    #[test]
    fn t_7470_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7470),-8);
    }
    #[test]
    fn t_7470_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7470),4);
    }
    #[test]
    fn t_7470_sen_test(){
        assert_eq!(test_sen_money_number(7470),-6);
    }
    #[test]
    fn t_7470_gosen_test(){
        assert_eq!(test_gosen_money_number(7470),-6);
    }
    #[test]
    fn t_7470_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7470),-4);
    }
    // 7480
    #[test]
    fn t_7480_ju_test(){
        assert_eq!(test_ju_money_number(7480),10);
    }
    #[test]
    fn t_7480_goju_test(){
        assert_eq!(test_goju_money_number(7480),-2);
    }
    #[test]
    fn t_7480_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7480),-8);
    }
    #[test]
    fn t_7480_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7480),4);
    }
    #[test]
    fn t_7480_sen_test(){
        assert_eq!(test_sen_money_number(7480),-6);
    }
    #[test]
    fn t_7480_gosen_test(){
        assert_eq!(test_gosen_money_number(7480),-6);
    }
    #[test]
    fn t_7480_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7480),-4);
    }
    // 7490
    #[test]
    fn t_7490_ju_test(){
        assert_eq!(test_ju_money_number(7490),0);
    }
    #[test]
    fn t_7490_goju_test(){
        assert_eq!(test_goju_money_number(7490),-2);
    }
    #[test]
    fn t_7490_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7490),-8);
    }
    #[test]
    fn t_7490_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7490),4);
    }
    #[test]
    fn t_7490_sen_test(){
        assert_eq!(test_sen_money_number(7490),-6);
    }
    #[test]
    fn t_7490_gosen_test(){
        assert_eq!(test_gosen_money_number(7490),-6);
    }
    #[test]
    fn t_7490_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7490),-4);
    }
    // 7500
    #[test]
    fn t_7500_ju_test(){
        assert_eq!(test_ju_money_number(7500),0);
    }
    #[test]
    fn t_7500_goju_test(){
        assert_eq!(test_goju_money_number(7500),0);
    }
    #[test]
    fn t_7500_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7500),0);
    }
    #[test]
    fn t_7500_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7500),4);
    }
    #[test]
    fn t_7500_sen_test(){
        assert_eq!(test_sen_money_number(7500),-7);
    }
    #[test]
    fn t_7500_gosen_test(){
        assert_eq!(test_gosen_money_number(7500),-6);
    }
    #[test]
    fn t_7500_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7500),-4);
    }
    // 7510
    #[test]
    fn t_7510_ju_test(){
        assert_eq!(test_ju_money_number(7510),30);
    }
    #[test]
    fn t_7510_goju_test(){
        assert_eq!(test_goju_money_number(7510),4);
    }
    #[test]
    fn t_7510_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7510),14);
    }
    #[test]
    fn t_7510_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7510),-6);
    }
    #[test]
    fn t_7510_sen_test(){
        assert_eq!(test_sen_money_number(7510),-14);
    }
    #[test]
    fn t_7510_gosen_test(){
        assert_eq!(test_gosen_money_number(7510),-8);
    }
    #[test]
    fn t_7510_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7510),-2);
    }
    // 7520
    #[test]
    fn t_7520_ju_test(){
        assert_eq!(test_ju_money_number(7520),20);
    }
    #[test]
    fn t_7520_goju_test(){
        assert_eq!(test_goju_money_number(7520),4);
    }
    #[test]
    fn t_7520_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7520),14);
    }
    #[test]
    fn t_7520_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7520),-6);
    }
    #[test]
    fn t_7520_sen_test(){
        assert_eq!(test_sen_money_number(7520),-14);
    }
    #[test]
    fn t_7520_gosen_test(){
        assert_eq!(test_gosen_money_number(7520),-8);
    }
    #[test]
    fn t_7520_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7520),-2);
    }
    // 7530
    #[test]
    fn t_7530_ju_test(){
        assert_eq!(test_ju_money_number(7530),10);
    }
    #[test]
    fn t_7530_goju_test(){
        assert_eq!(test_goju_money_number(7530),4);
    }
    #[test]
    fn t_7530_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7530),14);
    }
    #[test]
    fn t_7530_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7530),-6);
    }
    #[test]
    fn t_7530_sen_test(){
        assert_eq!(test_sen_money_number(7530),-14);
    }
    #[test]
    fn t_7530_gosen_test(){
        assert_eq!(test_gosen_money_number(7530),-8);
    }
    #[test]
    fn t_7530_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7530),-2);
    }
    // 7540
    #[test]
    fn t_7540_ju_test(){
        assert_eq!(test_ju_money_number(7540),0);
    }
    #[test]
    fn t_7540_goju_test(){
        assert_eq!(test_goju_money_number(7540),4);
    }
    #[test]
    fn t_7540_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7540),14);
    }
    #[test]
    fn t_7540_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7540),-6);
    }
    #[test]
    fn t_7540_sen_test(){
        assert_eq!(test_sen_money_number(7540),-14);
    }
    #[test]
    fn t_7540_gosen_test(){
        assert_eq!(test_gosen_money_number(7540),-8);
    }
    #[test]
    fn t_7540_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7540),-2);
    }
    // 7550
    #[test]
    fn t_7550_ju_test(){
        assert_eq!(test_ju_money_number(7550),0);
    }
    #[test]
    fn t_7550_goju_test(){
        assert_eq!(test_goju_money_number(7550),6);
    }
    #[test]
    fn t_7550_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7550),22);
    }
    #[test]
    fn t_7550_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7550),-4);
    }
    #[test]
    fn t_7550_sen_test(){
        assert_eq!(test_sen_money_number(7550),-6);
    }
    #[test]
    fn t_7550_gosen_test(){
        assert_eq!(test_gosen_money_number(7550),-6);
    }
    #[test]
    fn t_7550_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7550),-4);
    }
    // 7560
    #[test]
    fn t_7560_ju_test(){
        assert_eq!(test_ju_money_number(7560),30);
    }
    #[test]
    fn t_7560_goju_test(){
        assert_eq!(test_goju_money_number(7560),-2);
    }
    #[test]
    fn t_7560_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7560),22);
    }
    #[test]
    fn t_7560_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7560),-4);
    }
    #[test]
    fn t_7560_sen_test(){
        assert_eq!(test_sen_money_number(7560),-6);
    }
    #[test]
    fn t_7560_gosen_test(){
        assert_eq!(test_gosen_money_number(7560),-6);
    }
    #[test]
    fn t_7560_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7560),-4);
    }
    // 7570
    #[test]
    fn t_7570_ju_test(){
        assert_eq!(test_ju_money_number(7570),20);
    }
    #[test]
    fn t_7570_goju_test(){
        assert_eq!(test_goju_money_number(7570),-2);
    }
    #[test]
    fn t_7570_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7570),22);
    }
    #[test]
    fn t_7570_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7570),-4);
    }
    #[test]
    fn t_7570_sen_test(){
        assert_eq!(test_sen_money_number(7570),-6);
    }
    #[test]
    fn t_7570_gosen_test(){
        assert_eq!(test_gosen_money_number(7570),-6);
    }
    #[test]
    fn t_7570_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7570),-4);
    }
    // 7580
    #[test]
    fn t_7580_ju_test(){
        assert_eq!(test_ju_money_number(7580),10);
    }
    #[test]
    fn t_7580_goju_test(){
        assert_eq!(test_goju_money_number(7580),-2);
    }
    #[test]
    fn t_7580_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7580),22);
    }
    #[test]
    fn t_7580_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7580),-4);
    }
    #[test]
    fn t_7580_sen_test(){
        assert_eq!(test_sen_money_number(7580),-6);
    }
    #[test]
    fn t_7580_gosen_test(){
        assert_eq!(test_gosen_money_number(7580),-6);
    }
    #[test]
    fn t_7580_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7580),-4);
    }
    // 7590
    #[test]
    fn t_7590_ju_test(){
        assert_eq!(test_ju_money_number(7590),0);
    }
    #[test]
    fn t_7590_goju_test(){
        assert_eq!(test_goju_money_number(7590),-2);
    }
    #[test]
    fn t_7590_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7590),22);
    }
    #[test]
    fn t_7590_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7590),-4);
    }
    #[test]
    fn t_7590_sen_test(){
        assert_eq!(test_sen_money_number(7590),-6);
    }
    #[test]
    fn t_7590_gosen_test(){
        assert_eq!(test_gosen_money_number(7590),-6);
    }
    #[test]
    fn t_7590_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7590),-4);
    }
    // 7600
    #[test]
    fn t_7600_ju_test(){
        assert_eq!(test_ju_money_number(7600),0);
    }
    #[test]
    fn t_7600_goju_test(){
        assert_eq!(test_goju_money_number(7600),0);
    }
    #[test]
    fn t_7600_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7600),25);
    }
    #[test]
    fn t_7600_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7600),-3);
    }
    #[test]
    fn t_7600_sen_test(){
        assert_eq!(test_sen_money_number(7600),-7);
    }
    #[test]
    fn t_7600_gosen_test(){
        assert_eq!(test_gosen_money_number(7600),-6);
    }
    #[test]
    fn t_7600_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7600),-4);
    }
    // 7610
    #[test]
    fn t_7610_ju_test(){
        assert_eq!(test_ju_money_number(7610),30);
    }
    #[test]
    fn t_7610_goju_test(){
        assert_eq!(test_goju_money_number(7610),4);
    }
    #[test]
    fn t_7610_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7610),4);
    }
    #[test]
    fn t_7610_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7610),-6);
    }
    #[test]
    fn t_7610_sen_test(){
        assert_eq!(test_sen_money_number(7610),-14);
    }
    #[test]
    fn t_7610_gosen_test(){
        assert_eq!(test_gosen_money_number(7610),-8);
    }
    #[test]
    fn t_7610_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7610),-2);
    }
    // 7620
    #[test]
    fn t_7620_ju_test(){
        assert_eq!(test_ju_money_number(7620),20);
    }
    #[test]
    fn t_7620_goju_test(){
        assert_eq!(test_goju_money_number(7620),4);
    }
    #[test]
    fn t_7620_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7620),4);
    }
    #[test]
    fn t_7620_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7620),-6);
    }
    #[test]
    fn t_7620_sen_test(){
        assert_eq!(test_sen_money_number(7620),-14);
    }
    #[test]
    fn t_7620_gosen_test(){
        assert_eq!(test_gosen_money_number(7620),-8);
    }
    #[test]
    fn t_7620_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7620),-2);
    }
    // 7630
    #[test]
    fn t_7630_ju_test(){
        assert_eq!(test_ju_money_number(7630),10);
    }
    #[test]
    fn t_7630_goju_test(){
        assert_eq!(test_goju_money_number(7630),4);
    }
    #[test]
    fn t_7630_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7630),4);
    }
    #[test]
    fn t_7630_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7630),-6);
    }
    #[test]
    fn t_7630_sen_test(){
        assert_eq!(test_sen_money_number(7630),-14);
    }
    #[test]
    fn t_7630_gosen_test(){
        assert_eq!(test_gosen_money_number(7630),-8);
    }
    #[test]
    fn t_7630_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7630),-2);
    }
    // 7640
    #[test]
    fn t_7640_ju_test(){
        assert_eq!(test_ju_money_number(7640),0);
    }
    #[test]
    fn t_7640_goju_test(){
        assert_eq!(test_goju_money_number(7640),4);
    }
    #[test]
    fn t_7640_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7640),4);
    }
    #[test]
    fn t_7640_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7640),-6);
    }
    #[test]
    fn t_7640_sen_test(){
        assert_eq!(test_sen_money_number(7640),-14);
    }
    #[test]
    fn t_7640_gosen_test(){
        assert_eq!(test_gosen_money_number(7640),-8);
    }
    #[test]
    fn t_7640_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7640),-2);
    }
    // 7650
    #[test]
    fn t_7650_ju_test(){
        assert_eq!(test_ju_money_number(7650),0);
    }
    #[test]
    fn t_7650_goju_test(){
        assert_eq!(test_goju_money_number(7650),6);
    }
    #[test]
    fn t_7650_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7650),12);
    }
    #[test]
    fn t_7650_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7650),-4);
    }
    #[test]
    fn t_7650_sen_test(){
        assert_eq!(test_sen_money_number(7650),-6);
    }
    #[test]
    fn t_7650_gosen_test(){
        assert_eq!(test_gosen_money_number(7650),-6);
    }
    #[test]
    fn t_7650_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7650),-4);
    }
    // 7660
    #[test]
    fn t_7660_ju_test(){
        assert_eq!(test_ju_money_number(7660),30);
    }
    #[test]
    fn t_7660_goju_test(){
        assert_eq!(test_goju_money_number(7660),-2);
    }
    #[test]
    fn t_7660_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7660),12);
    }
    #[test]
    fn t_7660_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7660),-4);
    }
    #[test]
    fn t_7660_sen_test(){
        assert_eq!(test_sen_money_number(7660),-6);
    }
    #[test]
    fn t_7660_gosen_test(){
        assert_eq!(test_gosen_money_number(7660),-6);
    }
    #[test]
    fn t_7660_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7660),-4);
    }
    // 7670
    #[test]
    fn t_7670_ju_test(){
        assert_eq!(test_ju_money_number(7670),20);
    }
    #[test]
    fn t_7670_goju_test(){
        assert_eq!(test_goju_money_number(7670),-2);
    }
    #[test]
    fn t_7670_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7670),12);
    }
    #[test]
    fn t_7670_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7670),-4);
    }
    #[test]
    fn t_7670_sen_test(){
        assert_eq!(test_sen_money_number(7670),-6);
    }
    #[test]
    fn t_7670_gosen_test(){
        assert_eq!(test_gosen_money_number(7670),-6);
    }
    #[test]
    fn t_7670_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7670),-4);
    }
    // 7680
    #[test]
    fn t_7680_ju_test(){
        assert_eq!(test_ju_money_number(7680),10);
    }
    #[test]
    fn t_7680_goju_test(){
        assert_eq!(test_goju_money_number(7680),-2);
    }
    #[test]
    fn t_7680_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7680),12);
    }
    #[test]
    fn t_7680_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7680),-4);
    }
    #[test]
    fn t_7680_sen_test(){
        assert_eq!(test_sen_money_number(7680),-6);
    }
    #[test]
    fn t_7680_gosen_test(){
        assert_eq!(test_gosen_money_number(7680),-6);
    }
    #[test]
    fn t_7680_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7680),-4);
    }
    // 7690
    #[test]
    fn t_7690_ju_test(){
        assert_eq!(test_ju_money_number(7690),0);
    }
    #[test]
    fn t_7690_goju_test(){
        assert_eq!(test_goju_money_number(7690),-2);
    }
    #[test]
    fn t_7690_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7690),12);
    }
    #[test]
    fn t_7690_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7690),-4);
    }
    #[test]
    fn t_7690_sen_test(){
        assert_eq!(test_sen_money_number(7690),-6);
    }
    #[test]
    fn t_7690_gosen_test(){
        assert_eq!(test_gosen_money_number(7690),-6);
    }
    #[test]
    fn t_7690_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7690),-4);
    }
    // 7700
    #[test]
    fn t_7700_ju_test(){
        assert_eq!(test_ju_money_number(7700),0);
    }
    #[test]
    fn t_7700_goju_test(){
        assert_eq!(test_goju_money_number(7700),0);
    }
    #[test]
    fn t_7700_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7700),15);
    }
    #[test]
    fn t_7700_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7700),-3);
    }
    #[test]
    fn t_7700_sen_test(){
        assert_eq!(test_sen_money_number(7700),-7);
    }
    #[test]
    fn t_7700_gosen_test(){
        assert_eq!(test_gosen_money_number(7700),-6);
    }
    #[test]
    fn t_7700_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7700),-4);
    }
    // 7710
    #[test]
    fn t_7710_ju_test(){
        assert_eq!(test_ju_money_number(7710),30);
    }
    #[test]
    fn t_7710_goju_test(){
        assert_eq!(test_goju_money_number(7710),4);
    }
    #[test]
    fn t_7710_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7710),-6);
    }
    #[test]
    fn t_7710_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7710),-6);
    }
    #[test]
    fn t_7710_sen_test(){
        assert_eq!(test_sen_money_number(7710),-14);
    }
    #[test]
    fn t_7710_gosen_test(){
        assert_eq!(test_gosen_money_number(7710),-8);
    }
    #[test]
    fn t_7710_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7710),-2);
    }
    // 7720
    #[test]
    fn t_7720_ju_test(){
        assert_eq!(test_ju_money_number(7720),20);
    }
    #[test]
    fn t_7720_goju_test(){
        assert_eq!(test_goju_money_number(7720),4);
    }
    #[test]
    fn t_7720_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7720),-6);
    }
    #[test]
    fn t_7720_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7720),-6);
    }
    #[test]
    fn t_7720_sen_test(){
        assert_eq!(test_sen_money_number(7720),-14);
    }
    #[test]
    fn t_7720_gosen_test(){
        assert_eq!(test_gosen_money_number(7720),-8);
    }
    #[test]
    fn t_7720_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7720),-2);
    }
    // 7730
    #[test]
    fn t_7730_ju_test(){
        assert_eq!(test_ju_money_number(7730),10);
    }
    #[test]
    fn t_7730_goju_test(){
        assert_eq!(test_goju_money_number(7730),4);
    }
    #[test]
    fn t_7730_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7730),-6);
    }
    #[test]
    fn t_7730_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7730),-6);
    }
    #[test]
    fn t_7730_sen_test(){
        assert_eq!(test_sen_money_number(7730),-14);
    }
    #[test]
    fn t_7730_gosen_test(){
        assert_eq!(test_gosen_money_number(7730),-8);
    }
    #[test]
    fn t_7730_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7730),-2);
    }
    // 7740
    #[test]
    fn t_7740_ju_test(){
        assert_eq!(test_ju_money_number(7740),0);
    }
    #[test]
    fn t_7740_goju_test(){
        assert_eq!(test_goju_money_number(7740),4);
    }
    #[test]
    fn t_7740_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7740),-6);
    }
    #[test]
    fn t_7740_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7740),-6);
    }
    #[test]
    fn t_7740_sen_test(){
        assert_eq!(test_sen_money_number(7740),-14);
    }
    #[test]
    fn t_7740_gosen_test(){
        assert_eq!(test_gosen_money_number(7740),-8);
    }
    #[test]
    fn t_7740_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7740),-2);
    }
    // 7750
    #[test]
    fn t_7750_ju_test(){
        assert_eq!(test_ju_money_number(7750),0);
    }
    #[test]
    fn t_7750_goju_test(){
        assert_eq!(test_goju_money_number(7750),6);
    }
    #[test]
    fn t_7750_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7750),2);
    }
    #[test]
    fn t_7750_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7750),-4);
    }
    #[test]
    fn t_7750_sen_test(){
        assert_eq!(test_sen_money_number(7750),-6);
    }
    #[test]
    fn t_7750_gosen_test(){
        assert_eq!(test_gosen_money_number(7750),-6);
    }
    #[test]
    fn t_7750_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7750),-4);
    }
    // 7760
    #[test]
    fn t_7760_ju_test(){
        assert_eq!(test_ju_money_number(7760),30);
    }
    #[test]
    fn t_7760_goju_test(){
        assert_eq!(test_goju_money_number(7760),-2);
    }
    #[test]
    fn t_7760_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7760),2);
    }
    #[test]
    fn t_7760_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7760),-4);
    }
    #[test]
    fn t_7760_sen_test(){
        assert_eq!(test_sen_money_number(7760),-6);
    }
    #[test]
    fn t_7760_gosen_test(){
        assert_eq!(test_gosen_money_number(7760),-6);
    }
    #[test]
    fn t_7760_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7760),-4);
    }
    // 7770
    #[test]
    fn t_7770_ju_test(){
        assert_eq!(test_ju_money_number(7770),20);
    }
    #[test]
    fn t_7770_goju_test(){
        assert_eq!(test_goju_money_number(7770),-2);
    }
    #[test]
    fn t_7770_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7770),2);
    }
    #[test]
    fn t_7770_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7770),-4);
    }
    #[test]
    fn t_7770_sen_test(){
        assert_eq!(test_sen_money_number(7770),-6);
    }
    #[test]
    fn t_7770_gosen_test(){
        assert_eq!(test_gosen_money_number(7770),-6);
    }
    #[test]
    fn t_7770_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7770),-4);
    }
    // 7780
    #[test]
    fn t_7780_ju_test(){
        assert_eq!(test_ju_money_number(7780),10);
    }
    #[test]
    fn t_7780_goju_test(){
        assert_eq!(test_goju_money_number(7780),-2);
    }
    #[test]
    fn t_7780_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7780),2);
    }
    #[test]
    fn t_7780_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7780),-4);
    }
    #[test]
    fn t_7780_sen_test(){
        assert_eq!(test_sen_money_number(7780),-6);
    }
    #[test]
    fn t_7780_gosen_test(){
        assert_eq!(test_gosen_money_number(7780),-6);
    }
    #[test]
    fn t_7780_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7780),-4);
    }
    // 7790
    #[test]
    fn t_7790_ju_test(){
        assert_eq!(test_ju_money_number(7790),0);
    }
    #[test]
    fn t_7790_goju_test(){
        assert_eq!(test_goju_money_number(7790),-2);
    }
    #[test]
    fn t_7790_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7790),2);
    }
    #[test]
    fn t_7790_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7790),-4);
    }
    #[test]
    fn t_7790_sen_test(){
        assert_eq!(test_sen_money_number(7790),-6);
    }
    #[test]
    fn t_7790_gosen_test(){
        assert_eq!(test_gosen_money_number(7790),-6);
    }
    #[test]
    fn t_7790_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7790),-4);
    }
    // 7800
    #[test]
    fn t_7800_ju_test(){
        assert_eq!(test_ju_money_number(7800),0);
    }
    #[test]
    fn t_7800_goju_test(){
        assert_eq!(test_goju_money_number(7800),0);
    }
    #[test]
    fn t_7800_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7800),5);
    }
    #[test]
    fn t_7800_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7800),-3);
    }
    #[test]
    fn t_7800_sen_test(){
        assert_eq!(test_sen_money_number(7800),-7);
    }
    #[test]
    fn t_7800_gosen_test(){
        assert_eq!(test_gosen_money_number(7800),-6);
    }
    #[test]
    fn t_7800_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7800),-4);
    }
    // 7810
    #[test]
    fn t_7810_ju_test(){
        assert_eq!(test_ju_money_number(7810),30);
    }
    #[test]
    fn t_7810_goju_test(){
        assert_eq!(test_goju_money_number(7810),4);
    }
    #[test]
    fn t_7810_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7810),-16);
    }
    #[test]
    fn t_7810_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7810),-6);
    }
    #[test]
    fn t_7810_sen_test(){
        assert_eq!(test_sen_money_number(7810),-14);
    }
    #[test]
    fn t_7810_gosen_test(){
        assert_eq!(test_gosen_money_number(7810),-8);
    }
    #[test]
    fn t_7810_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7810),-2);
    }
    // 7820
    #[test]
    fn t_7820_ju_test(){
        assert_eq!(test_ju_money_number(7820),20);
    }
    #[test]
    fn t_7820_goju_test(){
        assert_eq!(test_goju_money_number(7820),4);
    }
    #[test]
    fn t_7820_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7820),-16);
    }
    #[test]
    fn t_7820_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7820),-6);
    }
    #[test]
    fn t_7820_sen_test(){
        assert_eq!(test_sen_money_number(7820),-14);
    }
    #[test]
    fn t_7820_gosen_test(){
        assert_eq!(test_gosen_money_number(7820),-8);
    }
    #[test]
    fn t_7820_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7820),-2);
    }
    // 7830
    #[test]
    fn t_7830_ju_test(){
        assert_eq!(test_ju_money_number(7830),10);
    }
    #[test]
    fn t_7830_goju_test(){
        assert_eq!(test_goju_money_number(7830),4);
    }
    #[test]
    fn t_7830_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7830),-16);
    }
    #[test]
    fn t_7830_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7830),-6);
    }
    #[test]
    fn t_7830_sen_test(){
        assert_eq!(test_sen_money_number(7830),-14);
    }
    #[test]
    fn t_7830_gosen_test(){
        assert_eq!(test_gosen_money_number(7830),-8);
    }
    #[test]
    fn t_7830_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7830),-2);
    }
    // 7840
    #[test]
    fn t_7840_ju_test(){
        assert_eq!(test_ju_money_number(7840),0);
    }
    #[test]
    fn t_7840_goju_test(){
        assert_eq!(test_goju_money_number(7840),4);
    }
    #[test]
    fn t_7840_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7840),-16);
    }
    #[test]
    fn t_7840_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7840),-6);
    }
    #[test]
    fn t_7840_sen_test(){
        assert_eq!(test_sen_money_number(7840),-14);
    }
    #[test]
    fn t_7840_gosen_test(){
        assert_eq!(test_gosen_money_number(7840),-8);
    }
    #[test]
    fn t_7840_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7840),-2);
    }
    // 7850
    #[test]
    fn t_7850_ju_test(){
        assert_eq!(test_ju_money_number(7850),0);
    }
    #[test]
    fn t_7850_goju_test(){
        assert_eq!(test_goju_money_number(7850),6);
    }
    #[test]
    fn t_7850_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7850),-8);
    }
    #[test]
    fn t_7850_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7850),-4);
    }
    #[test]
    fn t_7850_sen_test(){
        assert_eq!(test_sen_money_number(7850),-6);
    }
    #[test]
    fn t_7850_gosen_test(){
        assert_eq!(test_gosen_money_number(7850),-6);
    }
    #[test]
    fn t_7850_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7850),-4);
    }
    // 7860
    #[test]
    fn t_7860_ju_test(){
        assert_eq!(test_ju_money_number(7860),30);
    }
    #[test]
    fn t_7860_goju_test(){
        assert_eq!(test_goju_money_number(7860),-2);
    }
    #[test]
    fn t_7860_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7860),-8);
    }
    #[test]
    fn t_7860_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7860),-4);
    }
    #[test]
    fn t_7860_sen_test(){
        assert_eq!(test_sen_money_number(7860),-6);
    }
    #[test]
    fn t_7860_gosen_test(){
        assert_eq!(test_gosen_money_number(7860),-6);
    }
    #[test]
    fn t_7860_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7860),-4);
    }
    // 7870
    #[test]
    fn t_7870_ju_test(){
        assert_eq!(test_ju_money_number(7870),20);
    }
    #[test]
    fn t_7870_goju_test(){
        assert_eq!(test_goju_money_number(7870),-2);
    }
    #[test]
    fn t_7870_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7870),-8);
    }
    #[test]
    fn t_7870_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7870),-4);
    }
    #[test]
    fn t_7870_sen_test(){
        assert_eq!(test_sen_money_number(7870),-6);
    }
    #[test]
    fn t_7870_gosen_test(){
        assert_eq!(test_gosen_money_number(7870),-6);
    }
    #[test]
    fn t_7870_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7870),-4);
    }
    // 7890
    #[test]
    fn t_7890_ju_test(){
        assert_eq!(test_ju_money_number(7890),0);
    }
    #[test]
    fn t_7890_goju_test(){
        assert_eq!(test_goju_money_number(7890),-2);
    }
    #[test]
    fn t_7890_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7890),-8);
    }
    #[test]
    fn t_7890_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7890),-4);
    }
    #[test]
    fn t_7890_sen_test(){
        assert_eq!(test_sen_money_number(7890),-6);
    }
    #[test]
    fn t_7890_gosen_test(){
        assert_eq!(test_gosen_money_number(7890),-6);
    }
    #[test]
    fn t_7890_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7890),-4);
    }
    // 7900
    #[test]
    fn t_7900_ju_test(){
        assert_eq!(test_ju_money_number(7900),0);
    }
    #[test]
    fn t_7900_goju_test(){
        assert_eq!(test_goju_money_number(7900),0);
    }
    #[test]
    fn t_7900_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7900),-5);
    }
    #[test]
    fn t_7900_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7900),-3);
    }
    #[test]
    fn t_7900_sen_test(){
        assert_eq!(test_sen_money_number(7900),-7);
    }
    #[test]
    fn t_7900_gosen_test(){
        assert_eq!(test_gosen_money_number(7900),-6);
    }
    #[test]
    fn t_7900_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7900),-4);
    }
    // 7910
    #[test]
    fn t_7910_ju_test(){
        assert_eq!(test_ju_money_number(7910),30);
    }
    #[test]
    fn t_7910_goju_test(){
        assert_eq!(test_goju_money_number(7910),4);
    }
    #[test]
    fn t_7910_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7910),-16);
    }
    #[test]
    fn t_7910_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7910),-4);
    }
    #[test]
    fn t_7910_sen_test(){
        assert_eq!(test_sen_money_number(7910),-6);
    }
    #[test]
    fn t_7910_gosen_test(){
        assert_eq!(test_gosen_money_number(7910),-6);
    }
    #[test]
    fn t_7910_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7910),-4);
    }
    // 7920
    #[test]
    fn t_7920_ju_test(){
        assert_eq!(test_ju_money_number(7920),20);
    }
    #[test]
    fn t_7920_goju_test(){
        assert_eq!(test_goju_money_number(7920),4);
    }
    #[test]
    fn t_7920_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7920),-16);
    }
    #[test]
    fn t_7920_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7920),-4);
    }
    #[test]
    fn t_7920_sen_test(){
        assert_eq!(test_sen_money_number(7920),-6);
    }
    #[test]
    fn t_7920_gosen_test(){
        assert_eq!(test_gosen_money_number(7920),-6);
    }
    #[test]
    fn t_7920_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7920),-4);
    }
    // 7930
    #[test]
    fn t_7930_ju_test(){
        assert_eq!(test_ju_money_number(7930),10);
    }
    #[test]
    fn t_7930_goju_test(){
        assert_eq!(test_goju_money_number(7930),4);
    }
    #[test]
    fn t_7930_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7930),-16);
    }
    #[test]
    fn t_7930_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7930),-4);
    }
    #[test]
    fn t_7930_sen_test(){
        assert_eq!(test_sen_money_number(7930),-6);
    }
    #[test]
    fn t_7930_gosen_test(){
        assert_eq!(test_gosen_money_number(7930),-6);
    }
    #[test]
    fn t_7930_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7930),-4);
    }
    // 7940
    #[test]
    fn t_7940_ju_test(){
        assert_eq!(test_ju_money_number(7940),0);
    }
    #[test]
    fn t_7940_goju_test(){
        assert_eq!(test_goju_money_number(7940),4);
    }
    #[test]
    fn t_7940_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7940),-16);
    }
    #[test]
    fn t_7940_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7940),-4);
    }
    #[test]
    fn t_7940_sen_test(){
        assert_eq!(test_sen_money_number(7940),-6);
    }
    #[test]
    fn t_7940_gosen_test(){
        assert_eq!(test_gosen_money_number(7940),-6);
    }
    #[test]
    fn t_7940_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7940),-4);
    }
    // 7950
    #[test]
    fn t_7950_ju_test(){
        assert_eq!(test_ju_money_number(7950),0);
    }
    #[test]
    fn t_7950_goju_test(){
        assert_eq!(test_goju_money_number(7950),4);
    }
    #[test]
    fn t_7950_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7950),-12);
    }
    #[test]
    fn t_7950_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7950),-3);
    }
    #[test]
    fn t_7950_sen_test(){
        assert_eq!(test_sen_money_number(7950),-7);
    }
    #[test]
    fn t_7950_gosen_test(){
        assert_eq!(test_gosen_money_number(7950),-6);
    }
    #[test]
    fn t_7950_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7950),-4);
    }
    // 7960
    #[test]
    fn t_7960_ju_test(){
        assert_eq!(test_ju_money_number(7960),25);
    }
    #[test]
    fn t_7960_goju_test(){
        assert_eq!(test_goju_money_number(7960),-3);
    }
    #[test]
    fn t_7960_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7960),-12);
    }
    #[test]
    fn t_7960_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7960),-3);
    }
    #[test]
    fn t_7960_sen_test(){
        assert_eq!(test_sen_money_number(7960),-7);
    }
    #[test]
    fn t_7960_gosen_test(){
        assert_eq!(test_gosen_money_number(7960),-6);
    }
    #[test]
    fn t_7960_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7960),-4);
    }
    // 7970
    #[test]
    fn t_7970_ju_test(){
        assert_eq!(test_ju_money_number(7970),15);
    }
    #[test]
    fn t_7970_goju_test(){
        assert_eq!(test_goju_money_number(7970),-3);
    }
    #[test]
    fn t_7970_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7970),-12);
    }
    #[test]
    fn t_7970_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7970),-3);
    }
    #[test]
    fn t_7970_sen_test(){
        assert_eq!(test_sen_money_number(7970),-7);
    }
    #[test]
    fn t_7970_gosen_test(){
        assert_eq!(test_gosen_money_number(7970),-6);
    }
    #[test]
    fn t_7970_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7970),-4);
    }
    // 7980
    #[test]
    fn t_7980_ju_test(){
        assert_eq!(test_ju_money_number(7980),5);
    }
    #[test]
    fn t_7980_goju_test(){
        assert_eq!(test_goju_money_number(7980),-3);
    }
    #[test]
    fn t_7980_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7980),-12);
    }
    #[test]
    fn t_7980_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7980),-3);
    }
    #[test]
    fn t_7980_sen_test(){
        assert_eq!(test_sen_money_number(7980),-7);
    }
    #[test]
    fn t_7980_gosen_test(){
        assert_eq!(test_gosen_money_number(7980),-6);
    }
    #[test]
    fn t_7980_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7980),-4);
    }
    // 7990
    #[test]
    fn t_7990_ju_test(){
        assert_eq!(test_ju_money_number(7990),-5);
    }
    #[test]
    fn t_7990_goju_test(){
        assert_eq!(test_goju_money_number(7990),-3);
    }
    #[test]
    fn t_7990_hyaku_test(){
        assert_eq!(test_hyaku_money_number(7990),-12);
    }
    #[test]
    fn t_7990_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(7990),-3);
    }
    #[test]
    fn t_7990_sen_test(){
        assert_eq!(test_sen_money_number(7990),-7);
    }
    #[test]
    fn t_7990_gosen_test(){
        assert_eq!(test_gosen_money_number(7990),-6);
    }
    #[test]
    fn t_7990_ichiman_test(){
        assert_eq!(test_ichiman_money_number(7990),-4);
    }
    // 8000
    #[test]
    fn t_8000_ju_test(){
        assert_eq!(test_ju_money_number(8000),0);
    }
    #[test]
    fn t_8000_goju_test(){
        assert_eq!(test_goju_money_number(8000),0);
    }
    #[test]
    fn t_8000_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8000),0);
    }
    #[test]
    fn t_8000_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8000),0);
    }
    #[test]
    fn t_8000_sen_test(){
        assert_eq!(test_sen_money_number(8000),-5);
    }
    #[test]
    fn t_8000_gosen_test(){
        assert_eq!(test_gosen_money_number(8000),-5);
    }
    #[test]
    fn t_8000_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8000),-5);
    }
    // 8010
    #[test]
    fn t_8010_ju_test(){
        assert_eq!(test_ju_money_number(8010),35);
    }
    #[test]
    fn t_8010_goju_test(){
        assert_eq!(test_goju_money_number(8010),7);
    }
    #[test]
    fn t_8010_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8010),27);
    }
    #[test]
    fn t_8010_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8010),5);
    }
    #[test]
    fn t_8010_sen_test(){
        assert_eq!(test_sen_money_number(8010),-11);
    }
    #[test]
    fn t_8010_gosen_test(){
        assert_eq!(test_gosen_money_number(8010),-5);
    }
    #[test]
    fn t_8010_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8010),-5);
    }
    // 8020
    #[test]
    fn t_8020_ju_test(){
        assert_eq!(test_ju_money_number(8020),25);
    }
    #[test]
    fn t_8020_goju_test(){
        assert_eq!(test_goju_money_number(8020),7);
    }
    #[test]
    fn t_8020_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8020),27);
    }
    #[test]
    fn t_8020_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8020),5);
    }
    #[test]
    fn t_8020_sen_test(){
        assert_eq!(test_sen_money_number(8020),-11);
    }
    #[test]
    fn t_8020_gosen_test(){
        assert_eq!(test_gosen_money_number(8020),-5);
    }
    #[test]
    fn t_8020_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8020),-5);
    }
    // 8030
    #[test]
    fn t_8030_ju_test(){
        assert_eq!(test_ju_money_number(8030),15);
    }
    #[test]
    fn t_8030_goju_test(){
        assert_eq!(test_goju_money_number(8030),7);
    }
    #[test]
    fn t_8030_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8030),27);
    }
    #[test]
    fn t_8030_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8030),5);
    }
    #[test]
    fn t_8030_sen_test(){
        assert_eq!(test_sen_money_number(8030),-11);
    }
    #[test]
    fn t_8030_gosen_test(){
        assert_eq!(test_gosen_money_number(8030),-5);
    }
    #[test]
    fn t_8030_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8030),-5);
    }
    // 8040
    #[test]
    fn t_8040_ju_test(){
        assert_eq!(test_ju_money_number(8040),5);
    }
    #[test]
    fn t_8040_goju_test(){
        assert_eq!(test_goju_money_number(8040),7);
    }
    #[test]
    fn t_8040_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8040),27);
    }
    #[test]
    fn t_8040_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8040),5);
    }
    #[test]
    fn t_8040_sen_test(){
        assert_eq!(test_sen_money_number(8040),-11);
    }
    #[test]
    fn t_8040_gosen_test(){
        assert_eq!(test_gosen_money_number(8040),-5);
    }
    #[test]
    fn t_8040_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8040),-5);
    }
    // 8050
    #[test]
    fn t_8050_ju_test(){
        assert_eq!(test_ju_money_number(8050),0);
    }
    #[test]
    fn t_8050_goju_test(){
        assert_eq!(test_goju_money_number(8050),6);
    }
    #[test]
    fn t_8050_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8050),22);
    }
    #[test]
    fn t_8050_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8050),2);
    }
    #[test]
    fn t_8050_sen_test(){
        assert_eq!(test_sen_money_number(8050),-24);
    }
    #[test]
    fn t_8050_gosen_test(){
        assert_eq!(test_gosen_money_number(8050),-8);
    }
    #[test]
    fn t_8050_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8050),-2);
    }
    // 8060
    #[test]
    fn t_8060_ju_test(){
        assert_eq!(test_ju_money_number(8060),30);
    }
    #[test]
    fn t_8060_goju_test(){
        assert_eq!(test_goju_money_number(8060),-2);
    }
    #[test]
    fn t_8060_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8060),22);
    }
    #[test]
    fn t_8060_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8060),2);
    }
    #[test]
    fn t_8060_sen_test(){
        assert_eq!(test_sen_money_number(8060),-24);
    }
    #[test]
    fn t_8060_gosen_test(){
        assert_eq!(test_gosen_money_number(8060),-8);
    }
    #[test]
    fn t_8060_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8060),-2);
    }
    // 8070
    #[test]
    fn t_8070_ju_test(){
        assert_eq!(test_ju_money_number(8070),20);
    }
    #[test]
    fn t_8070_goju_test(){
        assert_eq!(test_goju_money_number(8070),-2);
    }
    #[test]
    fn t_8070_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8070),22);
    }
    #[test]
    fn t_8070_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8070),2);
    }
    #[test]
    fn t_8070_sen_test(){
        assert_eq!(test_sen_money_number(8070),-24);
    }
    #[test]
    fn t_8070_gosen_test(){
        assert_eq!(test_gosen_money_number(8070),-8);
    }
    #[test]
    fn t_8070_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8070),-2);
    }
    // 8080
    #[test]
    fn t_8080_ju_test(){
        assert_eq!(test_ju_money_number(8080),10);
    }
    #[test]
    fn t_8080_goju_test(){
        assert_eq!(test_goju_money_number(8080),-2);
    }
    #[test]
    fn t_8080_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8080),22);
    }
    #[test]
    fn t_8080_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8080),2);
    }
    #[test]
    fn t_8080_sen_test(){
        assert_eq!(test_sen_money_number(8080),-24);
    }
    #[test]
    fn t_8080_gosen_test(){
        assert_eq!(test_gosen_money_number(8080),-8);
    }
    #[test]
    fn t_8080_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8080),-2);
    }
    // 8090
    #[test]
    fn t_8090_ju_test(){
        assert_eq!(test_ju_money_number(8090),0);
    }
    #[test]
    fn t_8090_goju_test(){
        assert_eq!(test_goju_money_number(8090),-2);
    }
    #[test]
    fn t_8090_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8090),22);
    }
    #[test]
    fn t_8090_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8090),2);
    }
    #[test]
    fn t_8090_sen_test(){
        assert_eq!(test_sen_money_number(8090),-24);
    }
    #[test]
    fn t_8090_gosen_test(){
        assert_eq!(test_gosen_money_number(8090),-8);
    }
    #[test]
    fn t_8090_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8090),-2);
    }
    // 8100
    #[test]
    fn t_8100_ju_test(){
        assert_eq!(test_ju_money_number(8100),0);
    }
    #[test]
    fn t_8100_goju_test(){
        assert_eq!(test_goju_money_number(8100),0);
    }
    #[test]
    fn t_8100_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8100),30);
    }
    #[test]
    fn t_8100_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8100),4);
    }
    #[test]
    fn t_8100_sen_test(){
        assert_eq!(test_sen_money_number(8100),-16);
    }
    #[test]
    fn t_8100_gosen_test(){
        assert_eq!(test_gosen_money_number(8100),-6);
    }
    #[test]
    fn t_8100_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8100),-4);
    }
    // 8110
    #[test]
    fn t_8110_ju_test(){
        assert_eq!(test_ju_money_number(8110),35);
    }
    #[test]
    fn t_8110_goju_test(){
        assert_eq!(test_goju_money_number(8110),7);
    }
    #[test]
    fn t_8110_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8110),17);
    }
    #[test]
    fn t_8110_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8110),5);
    }
    #[test]
    fn t_8110_sen_test(){
        assert_eq!(test_sen_money_number(8110),-11);
    }
    #[test]
    fn t_8110_gosen_test(){
        assert_eq!(test_gosen_money_number(8110),-5);
    }
    #[test]
    fn t_8110_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8110),-5);
    }
    // 8120
    #[test]
    fn t_8120_ju_test(){
        assert_eq!(test_ju_money_number(8120),25);
    }
    #[test]
    fn t_8120_goju_test(){
        assert_eq!(test_goju_money_number(8120),7);
    }
    #[test]
    fn t_8120_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8120),17);
    }
    #[test]
    fn t_8120_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8120),5);
    }
    #[test]
    fn t_8120_sen_test(){
        assert_eq!(test_sen_money_number(8120),-11);
    }
    #[test]
    fn t_8120_gosen_test(){
        assert_eq!(test_gosen_money_number(8120),-5);
    }
    #[test]
    fn t_8120_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8120),-5);
    }
    // 8130
    #[test]
    fn t_8130_ju_test(){
        assert_eq!(test_ju_money_number(8130),15);
    }
    #[test]
    fn t_8130_goju_test(){
        assert_eq!(test_goju_money_number(8130),7);
    }
    #[test]
    fn t_8130_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8130),17);
    }
    #[test]
    fn t_8130_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8130),5);
    }
    #[test]
    fn t_8130_sen_test(){
        assert_eq!(test_sen_money_number(8130),-11);
    }
    #[test]
    fn t_8130_gosen_test(){
        assert_eq!(test_gosen_money_number(8130),-5);
    }
    #[test]
    fn t_8130_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8130),-5);
    }
    // 8140
    #[test]
    fn t_8140_ju_test(){
        assert_eq!(test_ju_money_number(8140),5);
    }
    #[test]
    fn t_8140_goju_test(){
        assert_eq!(test_goju_money_number(8140),7);
    }
    #[test]
    fn t_8140_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8140),17);
    }
    #[test]
    fn t_8140_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8140),5);
    }
    #[test]
    fn t_8140_sen_test(){
        assert_eq!(test_sen_money_number(8140),-11);
    }
    #[test]
    fn t_8140_gosen_test(){
        assert_eq!(test_gosen_money_number(8140),-5);
    }
    #[test]
    fn t_8140_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8140),-5);
    }
    // 8150
    #[test]
    fn t_8150_ju_test(){
        assert_eq!(test_ju_money_number(8150),0);
    }
    #[test]
    fn t_8150_goju_test(){
        assert_eq!(test_goju_money_number(8150),6);
    }
    #[test]
    fn t_8150_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8150),12);
    }
    #[test]
    fn t_8150_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8150),2);
    }
    #[test]
    fn t_8150_sen_test(){
        assert_eq!(test_sen_money_number(8150),-24);
    }
    #[test]
    fn t_8150_gosen_test(){
        assert_eq!(test_gosen_money_number(8150),-8);
    }
    #[test]
    fn t_8150_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8150),-2);
    }
    // 8160
    #[test]
    fn t_8160_ju_test(){
        assert_eq!(test_ju_money_number(8160),30);
    }
    #[test]
    fn t_8160_goju_test(){
        assert_eq!(test_goju_money_number(8160),-2);
    }
    #[test]
    fn t_8160_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8160),12);
    }
    #[test]
    fn t_8160_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8160),2);
    }
    #[test]
    fn t_8160_sen_test(){
        assert_eq!(test_sen_money_number(8160),-24);
    }
    #[test]
    fn t_8160_gosen_test(){
        assert_eq!(test_gosen_money_number(8160),-8);
    }
    #[test]
    fn t_8160_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8160),-2);
    }
    // 8170
    #[test]
    fn t_8170_ju_test(){
        assert_eq!(test_ju_money_number(8170),20);
    }
    #[test]
    fn t_8170_goju_test(){
        assert_eq!(test_goju_money_number(8170),-2);
    }
    #[test]
    fn t_8170_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8170),12);
    }
    #[test]
    fn t_8170_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8170),2);
    }
    #[test]
    fn t_8170_sen_test(){
        assert_eq!(test_sen_money_number(8170),-24);
    }
    #[test]
    fn t_8170_gosen_test(){
        assert_eq!(test_gosen_money_number(8170),-8);
    }
    #[test]
    fn t_8170_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8170),-2);
    }
    // 8180
    #[test]
    fn t_8180_ju_test(){
        assert_eq!(test_ju_money_number(8180),10);
    }
    #[test]
    fn t_8180_goju_test(){
        assert_eq!(test_goju_money_number(8180),-2);
    }
    #[test]
    fn t_8180_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8180),12);
    }
    #[test]
    fn t_8180_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8180),2);
    }
    #[test]
    fn t_8180_sen_test(){
        assert_eq!(test_sen_money_number(8180),-24);
    }
    #[test]
    fn t_8180_gosen_test(){
        assert_eq!(test_gosen_money_number(8180),-8);
    }
    #[test]
    fn t_8180_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8180),-2);
    }
    // 8190
    #[test]
    fn t_8190_ju_test(){
        assert_eq!(test_ju_money_number(8190),0);
    }
    #[test]
    fn t_8190_goju_test(){
        assert_eq!(test_goju_money_number(8190),-2);
    }
    #[test]
    fn t_8190_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8190),12);
    }
    #[test]
    fn t_8190_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8190),2);
    }
    #[test]
    fn t_8190_sen_test(){
        assert_eq!(test_sen_money_number(8190),-24);
    }
    #[test]
    fn t_8190_gosen_test(){
        assert_eq!(test_gosen_money_number(8190),-8);
    }
    #[test]
    fn t_8190_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8190),-2);
    }
    // 8200
    #[test]
    fn t_8200_ju_test(){
        assert_eq!(test_ju_money_number(8200),0);
    }
    #[test]
    fn t_8200_goju_test(){
        assert_eq!(test_goju_money_number(8200),0);
    }
    #[test]
    fn t_8200_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8200),20);
    }
    #[test]
    fn t_8200_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8200),4);
    }
    #[test]
    fn t_8200_sen_test(){
        assert_eq!(test_sen_money_number(8200),-16);
    }
    #[test]
    fn t_8200_gosen_test(){
        assert_eq!(test_gosen_money_number(8200),-6);
    }
    #[test]
    fn t_8200_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8200),-4);
    }
    // 8210
    #[test]
    fn t_8210_ju_test(){
        assert_eq!(test_ju_money_number(8210),35);
    }
    #[test]
    fn t_8210_goju_test(){
        assert_eq!(test_goju_money_number(8210),7);
    }
    #[test]
    fn t_8210_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8210),7);
    }
    #[test]
    fn t_8210_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8210),5);
    }
    #[test]
    fn t_8210_sen_test(){
        assert_eq!(test_sen_money_number(8210),-11);
    }
    #[test]
    fn t_8210_gosen_test(){
        assert_eq!(test_gosen_money_number(8210),-5);
    }
    #[test]
    fn t_8210_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8210),-5);
    }
    // 8220
    #[test]
    fn t_8220_ju_test(){
        assert_eq!(test_ju_money_number(8220),25);
    }
    #[test]
    fn t_8220_goju_test(){
        assert_eq!(test_goju_money_number(8220),7);
    }
    #[test]
    fn t_8220_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8220),7);
    }
    #[test]
    fn t_8220_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8220),5);
    }
    #[test]
    fn t_8220_sen_test(){
        assert_eq!(test_sen_money_number(8220),-11);
    }
    #[test]
    fn t_8220_gosen_test(){
        assert_eq!(test_gosen_money_number(8220),-5);
    }
    #[test]
    fn t_8220_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8220),-5);
    }
    // 8230
    #[test]
    fn t_8230_ju_test(){
        assert_eq!(test_ju_money_number(8230),15);
    }
    #[test]
    fn t_8230_goju_test(){
        assert_eq!(test_goju_money_number(8230),7);
    }
    #[test]
    fn t_8230_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8230),7);
    }
    #[test]
    fn t_8230_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8230),5);
    }
    #[test]
    fn t_8230_sen_test(){
        assert_eq!(test_sen_money_number(8230),-11);
    }
    #[test]
    fn t_8230_gosen_test(){
        assert_eq!(test_gosen_money_number(8230),-5);
    }
    #[test]
    fn t_8230_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8230),-5);
    }
    // 8240
    #[test]
    fn t_8240_ju_test(){
        assert_eq!(test_ju_money_number(8240),5);
    }
    #[test]
    fn t_8240_goju_test(){
        assert_eq!(test_goju_money_number(8240),7);
    }
    #[test]
    fn t_8240_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8240),7);
    }
    #[test]
    fn t_8240_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8240),5);
    }
    #[test]
    fn t_8240_sen_test(){
        assert_eq!(test_sen_money_number(8240),-11);
    }
    #[test]
    fn t_8240_gosen_test(){
        assert_eq!(test_gosen_money_number(8240),-5);
    }
    #[test]
    fn t_8240_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8240),-5);
    }
    // 8250
    #[test]
    fn t_8250_ju_test(){
        assert_eq!(test_ju_money_number(8250),0);
    }
    #[test]
    fn t_8250_goju_test(){
        assert_eq!(test_goju_money_number(8250),6);
    }
    #[test]
    fn t_8250_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8250),2);
    }
    #[test]
    fn t_8250_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8250),2);
    }
    #[test]
    fn t_8250_sen_test(){
        assert_eq!(test_sen_money_number(8250),-24);
    }
    #[test]
    fn t_8250_gosen_test(){
        assert_eq!(test_gosen_money_number(8250),-8);
    }
    #[test]
    fn t_8250_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8250),-2);
    }
    // 8260
    #[test]
    fn t_8260_ju_test(){
        assert_eq!(test_ju_money_number(8260),30);
    }
    #[test]
    fn t_8260_goju_test(){
        assert_eq!(test_goju_money_number(8260),-2);
    }
    #[test]
    fn t_8260_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8260),2);
    }
    #[test]
    fn t_8260_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8260),2);
    }
    #[test]
    fn t_8260_sen_test(){
        assert_eq!(test_sen_money_number(8260),-24);
    }
    #[test]
    fn t_8260_gosen_test(){
        assert_eq!(test_gosen_money_number(8260),-8);
    }
    #[test]
    fn t_8260_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8260),-2);
    }
    // 8270
    #[test]
    fn t_8270_ju_test(){
        assert_eq!(test_ju_money_number(8270),20);
    }
    #[test]
    fn t_8270_goju_test(){
        assert_eq!(test_goju_money_number(8270),-2);
    }
    #[test]
    fn t_8270_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8270),2);
    }
    #[test]
    fn t_8270_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8270),2);
    }
    #[test]
    fn t_8270_sen_test(){
        assert_eq!(test_sen_money_number(8270),-24);
    }
    #[test]
    fn t_8270_gosen_test(){
        assert_eq!(test_gosen_money_number(8270),-8);
    }
    #[test]
    fn t_8270_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8270),-2);
    }
    // 8280
    #[test]
    fn t_8280_ju_test(){
        assert_eq!(test_ju_money_number(8280),10);
    }
    #[test]
    fn t_8280_goju_test(){
        assert_eq!(test_goju_money_number(8280),-2);
    }
    #[test]
    fn t_8280_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8280),2);
    }
    #[test]
    fn t_8280_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8280),2);
    }
    #[test]
    fn t_8280_sen_test(){
        assert_eq!(test_sen_money_number(8280),-24);
    }
    #[test]
    fn t_8280_gosen_test(){
        assert_eq!(test_gosen_money_number(8280),-8);
    }
    #[test]
    fn t_8280_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8280),-2);
    }
    // 8290
    #[test]
    fn t_8290_ju_test(){
        assert_eq!(test_ju_money_number(8290),0);
    }
    #[test]
    fn t_8290_goju_test(){
        assert_eq!(test_goju_money_number(8290),-2);
    }
    #[test]
    fn t_8290_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8290),2);
    }
    #[test]
    fn t_8290_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8290),2);
    }
    #[test]
    fn t_8290_sen_test(){
        assert_eq!(test_sen_money_number(8290),-24);
    }
    #[test]
    fn t_8290_gosen_test(){
        assert_eq!(test_gosen_money_number(8290),-8);
    }
    #[test]
    fn t_8290_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8290),-2);
    }
    // 8300
    #[test]
    fn t_8300_ju_test(){
        assert_eq!(test_ju_money_number(8300),0);
    }
    #[test]
    fn t_8300_goju_test(){
        assert_eq!(test_goju_money_number(8300),0);
    }
    #[test]
    fn t_8300_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8300),10);
    }
    #[test]
    fn t_8300_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8300),4);
    }
    #[test]
    fn t_8300_sen_test(){
        assert_eq!(test_sen_money_number(8300),-16);
    }
    #[test]
    fn t_8300_gosen_test(){
        assert_eq!(test_gosen_money_number(8300),-6);
    }
    #[test]
    fn t_8300_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8300),-4);
    }
    // 8310
    #[test]
    fn t_8310_ju_test(){
        assert_eq!(test_ju_money_number(8310),35);
    }
    #[test]
    fn t_8310_goju_test(){
        assert_eq!(test_goju_money_number(8310),7);
    }
    #[test]
    fn t_8310_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8310),-3);
    }
    #[test]
    fn t_8310_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8310),5);
    }
    #[test]
    fn t_8310_sen_test(){
        assert_eq!(test_sen_money_number(8310),-11);
    }
    #[test]
    fn t_8310_gosen_test(){
        assert_eq!(test_gosen_money_number(8310),-5);
    }
    #[test]
    fn t_8310_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8310),-5);
    }
    // 8320
    #[test]
    fn t_8320_ju_test(){
        assert_eq!(test_ju_money_number(8320),25);
    }
    #[test]
    fn t_8320_goju_test(){
        assert_eq!(test_goju_money_number(8320),7);
    }
    #[test]
    fn t_8320_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8320),-3);
    }
    #[test]
    fn t_8320_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8320),5);
    }
    #[test]
    fn t_8320_sen_test(){
        assert_eq!(test_sen_money_number(8320),-11);
    }
    #[test]
    fn t_8320_gosen_test(){
        assert_eq!(test_gosen_money_number(8320),-5);
    }
    #[test]
    fn t_8320_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8320),-5);
    }
    // 8330
    #[test]
    fn t_8330_ju_test(){
        assert_eq!(test_ju_money_number(8330),15);
    }
    #[test]
    fn t_8330_goju_test(){
        assert_eq!(test_goju_money_number(8330),7);
    }
    #[test]
    fn t_8330_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8330),-3);
    }
    #[test]
    fn t_8330_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8330),5);
    }
    #[test]
    fn t_8330_sen_test(){
        assert_eq!(test_sen_money_number(8330),-11);
    }
    #[test]
    fn t_8330_gosen_test(){
        assert_eq!(test_gosen_money_number(8330),-5);
    }
    #[test]
    fn t_8330_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8330),-5);
    }
    // 8340
    #[test]
    fn t_8340_ju_test(){
        assert_eq!(test_ju_money_number(8340),5);
    }
    #[test]
    fn t_8340_goju_test(){
        assert_eq!(test_goju_money_number(8340),7);
    }
    #[test]
    fn t_8340_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8340),-3);
    }
    #[test]
    fn t_8340_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8340),5);
    }
    #[test]
    fn t_8340_sen_test(){
        assert_eq!(test_sen_money_number(8340),-11);
    }
    #[test]
    fn t_8340_gosen_test(){
        assert_eq!(test_gosen_money_number(8340),-5);
    }
    #[test]
    fn t_8340_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8340),-5);
    }
    // 8350
    #[test]
    fn t_8350_ju_test(){
        assert_eq!(test_ju_money_number(8350),0);
    }
    #[test]
    fn t_8350_goju_test(){
        assert_eq!(test_goju_money_number(8350),6);
    }
    #[test]
    fn t_8350_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8350),-8);
    }
    #[test]
    fn t_8350_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8350),2);
    }
    #[test]
    fn t_8350_sen_test(){
        assert_eq!(test_sen_money_number(8350),-24);
    }
    #[test]
    fn t_8350_gosen_test(){
        assert_eq!(test_gosen_money_number(8350),-8);
    }
    #[test]
    fn t_8350_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8350),-2);
    }
    // 8360
    #[test]
    fn t_8360_ju_test(){
        assert_eq!(test_ju_money_number(8360),30);
    }
    #[test]
    fn t_8360_goju_test(){
        assert_eq!(test_goju_money_number(8360),-2);
    }
    #[test]
    fn t_8360_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8360),-8);
    }
    #[test]
    fn t_8360_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8360),2);
    }
    #[test]
    fn t_8360_sen_test(){
        assert_eq!(test_sen_money_number(8360),-24);
    }
    #[test]
    fn t_8360_gosen_test(){
        assert_eq!(test_gosen_money_number(8360),-8);
    }
    #[test]
    fn t_8360_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8360),-2);
    }
    // 8370
    #[test]
    fn t_8370_ju_test(){
        assert_eq!(test_ju_money_number(8370),20);
    }
    #[test]
    fn t_8370_goju_test(){
        assert_eq!(test_goju_money_number(8370),-2);
    }
    #[test]
    fn t_8370_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8370),-8);
    }
    #[test]
    fn t_8370_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8370),2);
    }
    #[test]
    fn t_8370_sen_test(){
        assert_eq!(test_sen_money_number(8370),-24);
    }
    #[test]
    fn t_8370_gosen_test(){
        assert_eq!(test_gosen_money_number(8370),-8);
    }
    #[test]
    fn t_8370_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8370),-2);
    }
    // 8380
    #[test]
    fn t_8380_ju_test(){
        assert_eq!(test_ju_money_number(8380),10);
    }
    #[test]
    fn t_8380_goju_test(){
        assert_eq!(test_goju_money_number(8380),-2);
    }
    #[test]
    fn t_8380_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8380),-8);
    }
    #[test]
    fn t_8380_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8380),2);
    }
    #[test]
    fn t_8380_sen_test(){
        assert_eq!(test_sen_money_number(8380),-24);
    }
    #[test]
    fn t_8380_gosen_test(){
        assert_eq!(test_gosen_money_number(8380),-8);
    }
    #[test]
    fn t_8380_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8380),-2);
    }
    // 8390
    #[test]
    fn t_8390_ju_test(){
        assert_eq!(test_ju_money_number(8390),0);
    }
    #[test]
    fn t_8390_goju_test(){
        assert_eq!(test_goju_money_number(8390),-2);
    }
    #[test]
    fn t_8390_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8390),-8);
    }
    #[test]
    fn t_8390_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8390),2);
    }
    #[test]
    fn t_8390_sen_test(){
        assert_eq!(test_sen_money_number(8390),-24);
    }
    #[test]
    fn t_8390_gosen_test(){
        assert_eq!(test_gosen_money_number(8390),-8);
    }
    #[test]
    fn t_8390_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8390),-2);
    }
    // 8400
    #[test]
    fn t_8400_ju_test(){
        assert_eq!(test_ju_money_number(8400),0);
    }
    #[test]
    fn t_8400_goju_test(){
        assert_eq!(test_goju_money_number(8400),0);
    }
    #[test]
    fn t_8400_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8400),0);
    }
    #[test]
    fn t_8400_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8400),4);
    }
    #[test]
    fn t_8400_sen_test(){
        assert_eq!(test_sen_money_number(8400),-16);
    }
    #[test]
    fn t_8400_gosen_test(){
        assert_eq!(test_gosen_money_number(8400),-6);
    }
    #[test]
    fn t_8400_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8400),-4);
    }
    // 8410
    #[test]
    fn t_8410_ju_test(){
        assert_eq!(test_ju_money_number(8410),30);
    }
    #[test]
    fn t_8410_goju_test(){
        assert_eq!(test_goju_money_number(8410),4);
    }
    #[test]
    fn t_8410_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8410),-16);
    }
    #[test]
    fn t_8410_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8410),2);
    }
    #[test]
    fn t_8410_sen_test(){
        assert_eq!(test_sen_money_number(8410),-24);
    }
    #[test]
    fn t_8410_gosen_test(){
        assert_eq!(test_gosen_money_number(8410),-8);
    }
    #[test]
    fn t_8410_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8410),-2);
    }
    // 8420
    #[test]
    fn t_8420_ju_test(){
        assert_eq!(test_ju_money_number(8420),20);
    }
    #[test]
    fn t_8420_goju_test(){
        assert_eq!(test_goju_money_number(8420),4);
    }
    #[test]
    fn t_8420_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8420),-16);
    }
    #[test]
    fn t_8420_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8420),2);
    }
    #[test]
    fn t_8420_sen_test(){
        assert_eq!(test_sen_money_number(8420),-24);
    }
    #[test]
    fn t_8420_gosen_test(){
        assert_eq!(test_gosen_money_number(8420),-8);
    }
    #[test]
    fn t_8420_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8420),-2);
    }
    // 8430
    #[test]
    fn t_8430_ju_test(){
        assert_eq!(test_ju_money_number(8430),10);
    }
    #[test]
    fn t_8430_goju_test(){
        assert_eq!(test_goju_money_number(8430),4);
    }
    #[test]
    fn t_8430_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8430),-16);
    }
    #[test]
    fn t_8430_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8430),2);
    }
    #[test]
    fn t_8430_sen_test(){
        assert_eq!(test_sen_money_number(8430),-24);
    }
    #[test]
    fn t_8430_gosen_test(){
        assert_eq!(test_gosen_money_number(8430),-8);
    }
    #[test]
    fn t_8430_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8430),-2);
    }
    // 8440
    #[test]
    fn t_8440_ju_test(){
        assert_eq!(test_ju_money_number(8440),0);
    }
    #[test]
    fn t_8440_goju_test(){
        assert_eq!(test_goju_money_number(8440),4);
    }
    #[test]
    fn t_8440_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8440),-16);
    }
    #[test]
    fn t_8440_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8440),2);
    }
    #[test]
    fn t_8440_sen_test(){
        assert_eq!(test_sen_money_number(8440),-24);
    }
    #[test]
    fn t_8440_gosen_test(){
        assert_eq!(test_gosen_money_number(8440),-8);
    }
    #[test]
    fn t_8440_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8440),-2);
    }
    // 8450
    #[test]
    fn t_8450_ju_test(){
        assert_eq!(test_ju_money_number(8450),0);
    }
    #[test]
    fn t_8450_goju_test(){
        assert_eq!(test_goju_money_number(8450),6);
    }
    #[test]
    fn t_8450_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8450),-8);
    }
    #[test]
    fn t_8450_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8450),4);
    }
    #[test]
    fn t_8450_sen_test(){
        assert_eq!(test_sen_money_number(8450),-16);
    }
    #[test]
    fn t_8450_gosen_test(){
        assert_eq!(test_gosen_money_number(8450),-6);
    }
    #[test]
    fn t_8450_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8450),-4);
    }
    // 8460
    #[test]
    fn t_8460_ju_test(){
        assert_eq!(test_ju_money_number(8460),30);
    }
    #[test]
    fn t_8460_goju_test(){
        assert_eq!(test_goju_money_number(8460),-2);
    }
    #[test]
    fn t_8460_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8460),-8);
    }
    #[test]
    fn t_8460_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8460),4);
    }
    #[test]
    fn t_8460_sen_test(){
        assert_eq!(test_sen_money_number(8460),-16);
    }
    #[test]
    fn t_8460_gosen_test(){
        assert_eq!(test_gosen_money_number(8460),-6);
    }
    #[test]
    fn t_8460_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8460),-4);
    }
    // 8470
    #[test]
    fn t_8470_ju_test(){
        assert_eq!(test_ju_money_number(8470),20);
    }
    #[test]
    fn t_8470_goju_test(){
        assert_eq!(test_goju_money_number(8470),-2);
    }
    #[test]
    fn t_8470_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8470),-8);
    }
    #[test]
    fn t_8470_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8470),4);
    }
    #[test]
    fn t_8470_sen_test(){
        assert_eq!(test_sen_money_number(8470),-16);
    }
    #[test]
    fn t_8470_gosen_test(){
        assert_eq!(test_gosen_money_number(8470),-6);
    }
    #[test]
    fn t_8470_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8470),-4);
    }
    // 8480
    #[test]
    fn t_8480_ju_test(){
        assert_eq!(test_ju_money_number(8480),10);
    }
    #[test]
    fn t_8480_goju_test(){
        assert_eq!(test_goju_money_number(8480),-2);
    }
    #[test]
    fn t_8480_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8480),-8);
    }
    #[test]
    fn t_8480_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8480),4);
    }
    #[test]
    fn t_8480_sen_test(){
        assert_eq!(test_sen_money_number(8480),-16);
    }
    #[test]
    fn t_8480_gosen_test(){
        assert_eq!(test_gosen_money_number(8480),-6);
    }
    #[test]
    fn t_8480_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8480),-4);
    }
    // 8490
    #[test]
    fn t_8490_ju_test(){
        assert_eq!(test_ju_money_number(8490),0);
    }
    #[test]
    fn t_8490_goju_test(){
        assert_eq!(test_goju_money_number(8490),-2);
    }
    #[test]
    fn t_8490_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8490),-8);
    }
    #[test]
    fn t_8490_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8490),4);
    }
    #[test]
    fn t_8490_sen_test(){
        assert_eq!(test_sen_money_number(8490),-16);
    }
    #[test]
    fn t_8490_gosen_test(){
        assert_eq!(test_gosen_money_number(8490),-6);
    }
    #[test]
    fn t_8490_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8490),-4);
    }
    // 8500
    #[test]
    fn t_8500_ju_test(){
        assert_eq!(test_ju_money_number(8500),0);
    }
    #[test]
    fn t_8500_goju_test(){
        assert_eq!(test_goju_money_number(8500),0);
    }
    #[test]
    fn t_8500_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8500),0);
    }
    #[test]
    fn t_8500_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8500),4);
    }
    #[test]
    fn t_8500_sen_test(){
        assert_eq!(test_sen_money_number(8500),-17);
    }
    #[test]
    fn t_8500_gosen_test(){
        assert_eq!(test_gosen_money_number(8500),-6);
    }
    #[test]
    fn t_8500_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8500),-4);
    }
    // 8510
    #[test]
    fn t_8510_ju_test(){
        assert_eq!(test_ju_money_number(8510),30);
    }
    #[test]
    fn t_8510_goju_test(){
        assert_eq!(test_goju_money_number(8510),4);
    }
    #[test]
    fn t_8510_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8510),14);
    }
    #[test]
    fn t_8510_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8510),-6);
    }
    #[test]
    fn t_8510_sen_test(){
        assert_eq!(test_sen_money_number(8510),-24);
    }
    #[test]
    fn t_8510_gosen_test(){
        assert_eq!(test_gosen_money_number(8510),-8);
    }
    #[test]
    fn t_8510_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8510),-2);
    }
    // 8520
    #[test]
    fn t_8520_ju_test(){
        assert_eq!(test_ju_money_number(8520),20);
    }
    #[test]
    fn t_8520_goju_test(){
        assert_eq!(test_goju_money_number(8520),4);
    }
    #[test]
    fn t_8520_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8520),14);
    }
    #[test]
    fn t_8520_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8520),-6);
    }
    #[test]
    fn t_8520_sen_test(){
        assert_eq!(test_sen_money_number(8520),-24);
    }
    #[test]
    fn t_8520_gosen_test(){
        assert_eq!(test_gosen_money_number(8520),-8);
    }
    #[test]
    fn t_8520_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8520),-2);
    }
    // 8530
    #[test]
    fn t_8530_ju_test(){
        assert_eq!(test_ju_money_number(8530),10);
    }
    #[test]
    fn t_8530_goju_test(){
        assert_eq!(test_goju_money_number(8530),4);
    }
    #[test]
    fn t_8530_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8530),14);
    }
    #[test]
    fn t_8530_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8530),-6);
    }
    #[test]
    fn t_8530_sen_test(){
        assert_eq!(test_sen_money_number(8530),-24);
    }
    #[test]
    fn t_8530_gosen_test(){
        assert_eq!(test_gosen_money_number(8530),-8);
    }
    #[test]
    fn t_8530_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8530),-2);
    }
    // 8540
    #[test]
    fn t_8540_ju_test(){
        assert_eq!(test_ju_money_number(8540),0);
    }
    #[test]
    fn t_8540_goju_test(){
        assert_eq!(test_goju_money_number(8540),4);
    }
    #[test]
    fn t_8540_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8540),14);
    }
    #[test]
    fn t_8540_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8540),-6);
    }
    #[test]
    fn t_8540_sen_test(){
        assert_eq!(test_sen_money_number(8540),-24);
    }
    #[test]
    fn t_8540_gosen_test(){
        assert_eq!(test_gosen_money_number(8540),-8);
    }
    #[test]
    fn t_8540_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8540),-2);
    }
    // 8550
    #[test]
    fn t_8550_ju_test(){
        assert_eq!(test_ju_money_number(8550),0);
    }
    #[test]
    fn t_8550_goju_test(){
        assert_eq!(test_goju_money_number(8550),6);
    }
    #[test]
    fn t_8550_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8550),22);
    }
    #[test]
    fn t_8550_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8550),-4);
    }
    #[test]
    fn t_8550_sen_test(){
        assert_eq!(test_sen_money_number(8550),-16);
    }
    #[test]
    fn t_8550_gosen_test(){
        assert_eq!(test_gosen_money_number(8550),-6);
    }
    #[test]
    fn t_8550_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8550),-4);
    }
    // 8560
    #[test]
    fn t_8560_ju_test(){
        assert_eq!(test_ju_money_number(8560),30);
    }
    #[test]
    fn t_8560_goju_test(){
        assert_eq!(test_goju_money_number(8560),-2);
    }
    #[test]
    fn t_8560_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8560),22);
    }
    #[test]
    fn t_8560_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8560),-4);
    }
    #[test]
    fn t_8560_sen_test(){
        assert_eq!(test_sen_money_number(8560),-16);
    }
    #[test]
    fn t_8560_gosen_test(){
        assert_eq!(test_gosen_money_number(8560),-6);
    }
    #[test]
    fn t_8560_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8560),-4);
    }
    // 8570
    #[test]
    fn t_8570_ju_test(){
        assert_eq!(test_ju_money_number(8570),20);
    }
    #[test]
    fn t_8570_goju_test(){
        assert_eq!(test_goju_money_number(8570),-2);
    }
    #[test]
    fn t_8570_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8570),22);
    }
    #[test]
    fn t_8570_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8570),-4);
    }
    #[test]
    fn t_8570_sen_test(){
        assert_eq!(test_sen_money_number(8570),-16);
    }
    #[test]
    fn t_8570_gosen_test(){
        assert_eq!(test_gosen_money_number(8570),-6);
    }
    #[test]
    fn t_8570_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8570),-4);
    }
    // 8580
    #[test]
    fn t_8580_ju_test(){
        assert_eq!(test_ju_money_number(8580),10);
    }
    #[test]
    fn t_8580_goju_test(){
        assert_eq!(test_goju_money_number(8580),-2);
    }
    #[test]
    fn t_8580_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8580),22);
    }
    #[test]
    fn t_8580_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8580),-4);
    }
    #[test]
    fn t_8580_sen_test(){
        assert_eq!(test_sen_money_number(8580),-16);
    }
    #[test]
    fn t_8580_gosen_test(){
        assert_eq!(test_gosen_money_number(8580),-6);
    }
    #[test]
    fn t_8580_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8580),-4);
    }
    // 8590
    #[test]
    fn t_8590_ju_test(){
        assert_eq!(test_ju_money_number(8590),0);
    }
    #[test]
    fn t_8590_goju_test(){
        assert_eq!(test_goju_money_number(8590),-2);
    }
    #[test]
    fn t_8590_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8590),22);
    }
    #[test]
    fn t_8590_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8590),-4);
    }
    #[test]
    fn t_8590_sen_test(){
        assert_eq!(test_sen_money_number(8590),-16);
    }
    #[test]
    fn t_8590_gosen_test(){
        assert_eq!(test_gosen_money_number(8590),-6);
    }
    #[test]
    fn t_8590_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8590),-4);
    }
    // 8600
    #[test]
    fn t_8600_ju_test(){
        assert_eq!(test_ju_money_number(8600),0);
    }
    #[test]
    fn t_8600_goju_test(){
        assert_eq!(test_goju_money_number(8600),0);
    }
    #[test]
    fn t_8600_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8600),25);
    }
    #[test]
    fn t_8600_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8600),-3);
    }
    #[test]
    fn t_8600_sen_test(){
        assert_eq!(test_sen_money_number(8600),-17);
    }
    #[test]
    fn t_8600_gosen_test(){
        assert_eq!(test_gosen_money_number(8600),-6);
    }
    #[test]
    fn t_8600_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8600),-4);
    }
    // 8610
    #[test]
    fn t_8610_ju_test(){
        assert_eq!(test_ju_money_number(8610),30);
    }
    #[test]
    fn t_8610_goju_test(){
        assert_eq!(test_goju_money_number(8610),4);
    }
    #[test]
    fn t_8610_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8610),4);
    }
    #[test]
    fn t_8610_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8610),-6);
    }
    #[test]
    fn t_8610_sen_test(){
        assert_eq!(test_sen_money_number(8610),-24);
    }
    #[test]
    fn t_8610_gosen_test(){
        assert_eq!(test_gosen_money_number(8610),-8);
    }
    #[test]
    fn t_8610_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8610),-2);
    }
    // 8620
    #[test]
    fn t_8620_ju_test(){
        assert_eq!(test_ju_money_number(8620),20);
    }
    #[test]
    fn t_8620_goju_test(){
        assert_eq!(test_goju_money_number(8620),4);
    }
    #[test]
    fn t_8620_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8620),4);
    }
    #[test]
    fn t_8620_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8620),-6);
    }
    #[test]
    fn t_8620_sen_test(){
        assert_eq!(test_sen_money_number(8620),-24);
    }
    #[test]
    fn t_8620_gosen_test(){
        assert_eq!(test_gosen_money_number(8620),-8);
    }
    #[test]
    fn t_8620_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8620),-2);
    }
    // 8630
    #[test]
    fn t_8630_ju_test(){
        assert_eq!(test_ju_money_number(8630),10);
    }
    #[test]
    fn t_8630_goju_test(){
        assert_eq!(test_goju_money_number(8630),4);
    }
    #[test]
    fn t_8630_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8630),4);
    }
    #[test]
    fn t_8630_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8630),-6);
    }
    #[test]
    fn t_8630_sen_test(){
        assert_eq!(test_sen_money_number(8630),-24);
    }
    #[test]
    fn t_8630_gosen_test(){
        assert_eq!(test_gosen_money_number(8630),-8);
    }
    #[test]
    fn t_8630_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8630),-2);
    }
    // 8640
    #[test]
    fn t_8640_ju_test(){
        assert_eq!(test_ju_money_number(8640),0);
    }
    #[test]
    fn t_8640_goju_test(){
        assert_eq!(test_goju_money_number(8640),4);
    }
    #[test]
    fn t_8640_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8640),4);
    }
    #[test]
    fn t_8640_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8640),-6);
    }
    #[test]
    fn t_8640_sen_test(){
        assert_eq!(test_sen_money_number(8640),-24);
    }
    #[test]
    fn t_8640_gosen_test(){
        assert_eq!(test_gosen_money_number(8640),-8);
    }
    #[test]
    fn t_8640_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8640),-2);
    }
    // 8650
    #[test]
    fn t_8650_ju_test(){
        assert_eq!(test_ju_money_number(8650),0);
    }
    #[test]
    fn t_8650_goju_test(){
        assert_eq!(test_goju_money_number(8650),6);
    }
    #[test]
    fn t_8650_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8650),12);
    }
    #[test]
    fn t_8650_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8650),-4);
    }
    #[test]
    fn t_8650_sen_test(){
        assert_eq!(test_sen_money_number(8650),-16);
    }
    #[test]
    fn t_8650_gosen_test(){
        assert_eq!(test_gosen_money_number(8650),-6);
    }
    #[test]
    fn t_8650_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8650),-4);
    }
    // 8660
    #[test]
    fn t_8660_ju_test(){
        assert_eq!(test_ju_money_number(8660),30);
    }
    #[test]
    fn t_8660_goju_test(){
        assert_eq!(test_goju_money_number(8660),-2);
    }
    #[test]
    fn t_8660_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8660),12);
    }
    #[test]
    fn t_8660_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8660),-4);
    }
    #[test]
    fn t_8660_sen_test(){
        assert_eq!(test_sen_money_number(8660),-16);
    }
    #[test]
    fn t_8660_gosen_test(){
        assert_eq!(test_gosen_money_number(8660),-6);
    }
    #[test]
    fn t_8660_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8660),-4);
    }
    // 8670
    #[test]
    fn t_8670_ju_test(){
        assert_eq!(test_ju_money_number(8670),20);
    }
    #[test]
    fn t_8670_goju_test(){
        assert_eq!(test_goju_money_number(8670),-2);
    }
    #[test]
    fn t_8670_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8670),12);
    }
    #[test]
    fn t_8670_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8670),-4);
    }
    #[test]
    fn t_8670_sen_test(){
        assert_eq!(test_sen_money_number(8670),-16);
    }
    #[test]
    fn t_8670_gosen_test(){
        assert_eq!(test_gosen_money_number(8670),-6);
    }
    #[test]
    fn t_8670_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8670),-4);
    }
    // 8680
    #[test]
    fn t_8680_ju_test(){
        assert_eq!(test_ju_money_number(8680),10);
    }
    #[test]
    fn t_8680_goju_test(){
        assert_eq!(test_goju_money_number(8680),-2);
    }
    #[test]
    fn t_8680_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8680),12);
    }
    #[test]
    fn t_8680_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8680),-4);
    }
    #[test]
    fn t_8680_sen_test(){
        assert_eq!(test_sen_money_number(8680),-16);
    }
    #[test]
    fn t_8680_gosen_test(){
        assert_eq!(test_gosen_money_number(8680),-6);
    }
    #[test]
    fn t_8680_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8680),-4);
    }
    // 8690
    #[test]
    fn t_8690_ju_test(){
        assert_eq!(test_ju_money_number(8690),0);
    }
    #[test]
    fn t_8690_goju_test(){
        assert_eq!(test_goju_money_number(8690),-2);
    }
    #[test]
    fn t_8690_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8690),12);
    }
    #[test]
    fn t_8690_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8690),-4);
    }
    #[test]
    fn t_8690_sen_test(){
        assert_eq!(test_sen_money_number(8690),-16);
    }
    #[test]
    fn t_8690_gosen_test(){
        assert_eq!(test_gosen_money_number(8690),-6);
    }
    #[test]
    fn t_8690_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8690),-4);
    }
    // 8700
    #[test]
    fn t_8700_ju_test(){
        assert_eq!(test_ju_money_number(8700),0);
    }
    #[test]
    fn t_8700_goju_test(){
        assert_eq!(test_goju_money_number(8700),0);
    }
    #[test]
    fn t_8700_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8700),15);
    }
    #[test]
    fn t_8700_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8700),-3);
    }
    #[test]
    fn t_8700_sen_test(){
        assert_eq!(test_sen_money_number(8700),-17);
    }
    #[test]
    fn t_8700_gosen_test(){
        assert_eq!(test_gosen_money_number(8700),-6);
    }
    #[test]
    fn t_8700_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8700),-4);
    }
    // 8710
    #[test]
    fn t_8710_ju_test(){
        assert_eq!(test_ju_money_number(8710),30);
    }
    #[test]
    fn t_8710_goju_test(){
        assert_eq!(test_goju_money_number(8710),4);
    }
    #[test]
    fn t_8710_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8710),-6);
    }
    #[test]
    fn t_8710_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8710),-6);
    }
    #[test]
    fn t_8710_sen_test(){
        assert_eq!(test_sen_money_number(8710),-24);
    }
    #[test]
    fn t_8710_gosen_test(){
        assert_eq!(test_gosen_money_number(8710),-8);
    }
    #[test]
    fn t_8710_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8710),-2);
    }
    // 8720
    #[test]
    fn t_8720_ju_test(){
        assert_eq!(test_ju_money_number(8720),20);
    }
    #[test]
    fn t_8720_goju_test(){
        assert_eq!(test_goju_money_number(8720),4);
    }
    #[test]
    fn t_8720_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8720),-6);
    }
    #[test]
    fn t_8720_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8720),-6);
    }
    #[test]
    fn t_8720_sen_test(){
        assert_eq!(test_sen_money_number(8720),-24);
    }
    #[test]
    fn t_8720_gosen_test(){
        assert_eq!(test_gosen_money_number(8720),-8);
    }
    #[test]
    fn t_8720_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8720),-2);
    }
    // 8730
    #[test]
    fn t_8730_ju_test(){
        assert_eq!(test_ju_money_number(8730),10);
    }
    #[test]
    fn t_8730_goju_test(){
        assert_eq!(test_goju_money_number(8730),4);
    }
    #[test]
    fn t_8730_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8730),-6);
    }
    #[test]
    fn t_8730_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8730),-6);
    }
    #[test]
    fn t_8730_sen_test(){
        assert_eq!(test_sen_money_number(8730),-24);
    }
    #[test]
    fn t_8730_gosen_test(){
        assert_eq!(test_gosen_money_number(8730),-8);
    }
    #[test]
    fn t_8730_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8730),-2);
    }
    // 8740
    #[test]
    fn t_8740_ju_test(){
        assert_eq!(test_ju_money_number(8740),0);
    }
    #[test]
    fn t_8740_goju_test(){
        assert_eq!(test_goju_money_number(8740),4);
    }
    #[test]
    fn t_8740_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8740),-6);
    }
    #[test]
    fn t_8740_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8740),-6);
    }
    #[test]
    fn t_8740_sen_test(){
        assert_eq!(test_sen_money_number(8740),-24);
    }
    #[test]
    fn t_8740_gosen_test(){
        assert_eq!(test_gosen_money_number(8740),-8);
    }
    #[test]
    fn t_8740_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8740),-2);
    }
    // 8750
    #[test]
    fn t_8750_ju_test(){
        assert_eq!(test_ju_money_number(8750),0);
    }
    #[test]
    fn t_8750_goju_test(){
        assert_eq!(test_goju_money_number(8750),6);
    }
    #[test]
    fn t_8750_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8750),2);
    }
    #[test]
    fn t_8750_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8750),-4);
    }
    #[test]
    fn t_8750_sen_test(){
        assert_eq!(test_sen_money_number(8750),-16);
    }
    #[test]
    fn t_8750_gosen_test(){
        assert_eq!(test_gosen_money_number(8750),-6);
    }
    #[test]
    fn t_8750_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8750),-4);
    }
    // 8760
    #[test]
    fn t_8760_ju_test(){
        assert_eq!(test_ju_money_number(8760),30);
    }
    #[test]
    fn t_8760_goju_test(){
        assert_eq!(test_goju_money_number(8760),-2);
    }
    #[test]
    fn t_8760_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8760),2);
    }
    #[test]
    fn t_8760_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8760),-4);
    }
    #[test]
    fn t_8760_sen_test(){
        assert_eq!(test_sen_money_number(8760),-16);
    }
    #[test]
    fn t_8760_gosen_test(){
        assert_eq!(test_gosen_money_number(8760),-6);
    }
    #[test]
    fn t_8760_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8760),-4);
    }
    // 8770
    #[test]
    fn t_8770_ju_test(){
        assert_eq!(test_ju_money_number(8770),20);
    }
    #[test]
    fn t_8770_goju_test(){
        assert_eq!(test_goju_money_number(8770),-2);
    }
    #[test]
    fn t_8770_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8770),2);
    }
    #[test]
    fn t_8770_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8770),-4);
    }
    #[test]
    fn t_8770_sen_test(){
        assert_eq!(test_sen_money_number(8770),-16);
    }
    #[test]
    fn t_8770_gosen_test(){
        assert_eq!(test_gosen_money_number(8770),-6);
    }
    #[test]
    fn t_8770_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8770),-4);
    }
    // 8780
    #[test]
    fn t_8780_ju_test(){
        assert_eq!(test_ju_money_number(8780),10);
    }
    #[test]
    fn t_8780_goju_test(){
        assert_eq!(test_goju_money_number(8780),-2);
    }
    #[test]
    fn t_8780_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8780),2);
    }
    #[test]
    fn t_8780_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8780),-4);
    }
    #[test]
    fn t_8780_sen_test(){
        assert_eq!(test_sen_money_number(8780),-16);
    }
    #[test]
    fn t_8780_gosen_test(){
        assert_eq!(test_gosen_money_number(8780),-6);
    }
    #[test]
    fn t_8780_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8780),-4);
    }
    // 8790
    #[test]
    fn t_8790_ju_test(){
        assert_eq!(test_ju_money_number(8790),0);
    }
    #[test]
    fn t_8790_goju_test(){
        assert_eq!(test_goju_money_number(8790),-2);
    }
    #[test]
    fn t_8790_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8790),2);
    }
    #[test]
    fn t_8790_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8790),-4);
    }
    #[test]
    fn t_8790_sen_test(){
        assert_eq!(test_sen_money_number(8790),-16);
    }
    #[test]
    fn t_8790_gosen_test(){
        assert_eq!(test_gosen_money_number(8790),-6);
    }
    #[test]
    fn t_8790_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8790),-4);
    }
    // 8800
    #[test]
    fn t_8800_ju_test(){
        assert_eq!(test_ju_money_number(8800),0);
    }
    #[test]
    fn t_8800_goju_test(){
        assert_eq!(test_goju_money_number(8800),0);
    }
    #[test]
    fn t_8800_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8800),5);
    }
    #[test]
    fn t_8800_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8800),-3);
    }
    #[test]
    fn t_8800_sen_test(){
        assert_eq!(test_sen_money_number(8800),-17);
    }
    #[test]
    fn t_8800_gosen_test(){
        assert_eq!(test_gosen_money_number(8800),-6);
    }
    #[test]
    fn t_8800_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8800),-4);
    }
    // 8810
    #[test]
    fn t_8810_ju_test(){
        assert_eq!(test_ju_money_number(8810),30);
    }
    #[test]
    fn t_8810_goju_test(){
        assert_eq!(test_goju_money_number(8810),4);
    }
    #[test]
    fn t_8810_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8810),-16);
    }
    #[test]
    fn t_8810_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8810),-6);
    }
    #[test]
    fn t_8810_sen_test(){
        assert_eq!(test_sen_money_number(8810),-24);
    }
    #[test]
    fn t_8810_gosen_test(){
        assert_eq!(test_gosen_money_number(8810),-8);
    }
    #[test]
    fn t_8810_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8810),-2);
    }
    // 8820
    #[test]
    fn t_8820_ju_test(){
        assert_eq!(test_ju_money_number(8820),20);
    }
    #[test]
    fn t_8820_goju_test(){
        assert_eq!(test_goju_money_number(8820),4);
    }
    #[test]
    fn t_8820_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8820),-16);
    }
    #[test]
    fn t_8820_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8820),-6);
    }
    #[test]
    fn t_8820_sen_test(){
        assert_eq!(test_sen_money_number(8820),-24);
    }
    #[test]
    fn t_8820_gosen_test(){
        assert_eq!(test_gosen_money_number(8820),-8);
    }
    #[test]
    fn t_8820_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8820),-2);
    }
    // 8830
    #[test]
    fn t_8830_ju_test(){
        assert_eq!(test_ju_money_number(8830),10);
    }
    #[test]
    fn t_8830_goju_test(){
        assert_eq!(test_goju_money_number(8830),4);
    }
    #[test]
    fn t_8830_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8830),-16);
    }
    #[test]
    fn t_8830_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8830),-6);
    }
    #[test]
    fn t_8830_sen_test(){
        assert_eq!(test_sen_money_number(8830),-24);
    }
    #[test]
    fn t_8830_gosen_test(){
        assert_eq!(test_gosen_money_number(8830),-8);
    }
    #[test]
    fn t_8830_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8830),-2);
    }
    // 8840
    #[test]
    fn t_8840_ju_test(){
        assert_eq!(test_ju_money_number(8840),0);
    }
    #[test]
    fn t_8840_goju_test(){
        assert_eq!(test_goju_money_number(8840),4);
    }
    #[test]
    fn t_8840_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8840),-16);
    }
    #[test]
    fn t_8840_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8840),-6);
    }
    #[test]
    fn t_8840_sen_test(){
        assert_eq!(test_sen_money_number(8840),-24);
    }
    #[test]
    fn t_8840_gosen_test(){
        assert_eq!(test_gosen_money_number(8840),-8);
    }
    #[test]
    fn t_8840_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8840),-2);
    }
    // 8850
    #[test]
    fn t_8850_ju_test(){
        assert_eq!(test_ju_money_number(8850),0);
    }
    #[test]
    fn t_8850_goju_test(){
        assert_eq!(test_goju_money_number(8850),6);
    }
    #[test]
    fn t_8850_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8850),-8);
    }
    #[test]
    fn t_8850_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8850),-4);
    }
    #[test]
    fn t_8850_sen_test(){
        assert_eq!(test_sen_money_number(8850),-16);
    }
    #[test]
    fn t_8850_gosen_test(){
        assert_eq!(test_gosen_money_number(8850),-6);
    }
    #[test]
    fn t_8850_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8850),-4);
    }
    // 8860
    #[test]
    fn t_8860_ju_test(){
        assert_eq!(test_ju_money_number(8860),30);
    }
    #[test]
    fn t_8860_goju_test(){
        assert_eq!(test_goju_money_number(8860),-2);
    }
    #[test]
    fn t_8860_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8860),-8);
    }
    #[test]
    fn t_8860_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8860),-4);
    }
    #[test]
    fn t_8860_sen_test(){
        assert_eq!(test_sen_money_number(8860),-16);
    }
    #[test]
    fn t_8860_gosen_test(){
        assert_eq!(test_gosen_money_number(8860),-6);
    }
    #[test]
    fn t_8860_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8860),-4);
    }
    // 8870
    #[test]
    fn t_8870_ju_test(){
        assert_eq!(test_ju_money_number(8870),20);
    }
    #[test]
    fn t_8870_goju_test(){
        assert_eq!(test_goju_money_number(8870),-2);
    }
    #[test]
    fn t_8870_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8870),-8);
    }
    #[test]
    fn t_8870_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8870),-4);
    }
    #[test]
    fn t_8870_sen_test(){
        assert_eq!(test_sen_money_number(8870),-16);
    }
    #[test]
    fn t_8870_gosen_test(){
        assert_eq!(test_gosen_money_number(8870),-6);
    }
    #[test]
    fn t_8870_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8870),-4);
    }
    // 8890
    #[test]
    fn t_8890_ju_test(){
        assert_eq!(test_ju_money_number(8890),0);
    }
    #[test]
    fn t_8890_goju_test(){
        assert_eq!(test_goju_money_number(8890),-2);
    }
    #[test]
    fn t_8890_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8890),-8);
    }
    #[test]
    fn t_8890_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8890),-4);
    }
    #[test]
    fn t_8890_sen_test(){
        assert_eq!(test_sen_money_number(8890),-16);
    }
    #[test]
    fn t_8890_gosen_test(){
        assert_eq!(test_gosen_money_number(8890),-6);
    }
    #[test]
    fn t_8890_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8890),-4);
    }
    // 8900
    #[test]
    fn t_8900_ju_test(){
        assert_eq!(test_ju_money_number(8900),0);
    }
    #[test]
    fn t_8900_goju_test(){
        assert_eq!(test_goju_money_number(8900),0);
    }
    #[test]
    fn t_8900_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8900),-5);
    }
    #[test]
    fn t_8900_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8900),-3);
    }
    #[test]
    fn t_8900_sen_test(){
        assert_eq!(test_sen_money_number(8900),-17);
    }
    #[test]
    fn t_8900_gosen_test(){
        assert_eq!(test_gosen_money_number(8900),-6);
    }
    #[test]
    fn t_8900_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8900),-4);
    }
    // 8910
    #[test]
    fn t_8910_ju_test(){
        assert_eq!(test_ju_money_number(8910),30);
    }
    #[test]
    fn t_8910_goju_test(){
        assert_eq!(test_goju_money_number(8910),4);
    }
    #[test]
    fn t_8910_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8910),-16);
    }
    #[test]
    fn t_8910_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8910),-4);
    }
    #[test]
    fn t_8910_sen_test(){
        assert_eq!(test_sen_money_number(8910),-16);
    }
    #[test]
    fn t_8910_gosen_test(){
        assert_eq!(test_gosen_money_number(8910),-6);
    }
    #[test]
    fn t_8910_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8910),-4);
    }
    // 8920
    #[test]
    fn t_8920_ju_test(){
        assert_eq!(test_ju_money_number(8920),20);
    }
    #[test]
    fn t_8920_goju_test(){
        assert_eq!(test_goju_money_number(8920),4);
    }
    #[test]
    fn t_8920_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8920),-16);
    }
    #[test]
    fn t_8920_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8920),-4);
    }
    #[test]
    fn t_8920_sen_test(){
        assert_eq!(test_sen_money_number(8920),-16);
    }
    #[test]
    fn t_8920_gosen_test(){
        assert_eq!(test_gosen_money_number(8920),-6);
    }
    #[test]
    fn t_8920_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8920),-4);
    }
    // 8930
    #[test]
    fn t_8930_ju_test(){
        assert_eq!(test_ju_money_number(8930),10);
    }
    #[test]
    fn t_8930_goju_test(){
        assert_eq!(test_goju_money_number(8930),4);
    }
    #[test]
    fn t_8930_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8930),-16);
    }
    #[test]
    fn t_8930_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8930),-4);
    }
    #[test]
    fn t_8930_sen_test(){
        assert_eq!(test_sen_money_number(8930),-16);
    }
    #[test]
    fn t_8930_gosen_test(){
        assert_eq!(test_gosen_money_number(8930),-6);
    }
    #[test]
    fn t_8930_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8930),-4);
    }
    // 8940
    #[test]
    fn t_8940_ju_test(){
        assert_eq!(test_ju_money_number(8940),0);
    }
    #[test]
    fn t_8940_goju_test(){
        assert_eq!(test_goju_money_number(8940),4);
    }
    #[test]
    fn t_8940_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8940),-16);
    }
    #[test]
    fn t_8940_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8940),-4);
    }
    #[test]
    fn t_8940_sen_test(){
        assert_eq!(test_sen_money_number(8940),-16);
    }
    #[test]
    fn t_8940_gosen_test(){
        assert_eq!(test_gosen_money_number(8940),-6);
    }
    #[test]
    fn t_8940_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8940),-4);
    }
    // 8950
    #[test]
    fn t_8950_ju_test(){
        assert_eq!(test_ju_money_number(8950),0);
    }
    #[test]
    fn t_8950_goju_test(){
        assert_eq!(test_goju_money_number(8950),4);
    }
    #[test]
    fn t_8950_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8950),-12);
    }
    #[test]
    fn t_8950_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8950),-3);
    }
    #[test]
    fn t_8950_sen_test(){
        assert_eq!(test_sen_money_number(8950),-17);
    }
    #[test]
    fn t_8950_gosen_test(){
        assert_eq!(test_gosen_money_number(8950),-6);
    }
    #[test]
    fn t_8950_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8950),-4);
    }
    // 8960
    #[test]
    fn t_8960_ju_test(){
        assert_eq!(test_ju_money_number(8960),25);
    }
    #[test]
    fn t_8960_goju_test(){
        assert_eq!(test_goju_money_number(8960),-3);
    }
    #[test]
    fn t_8960_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8960),-12);
    }
    #[test]
    fn t_8960_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8960),-3);
    }
    #[test]
    fn t_8960_sen_test(){
        assert_eq!(test_sen_money_number(8960),-17);
    }
    #[test]
    fn t_8960_gosen_test(){
        assert_eq!(test_gosen_money_number(8960),-6);
    }
    #[test]
    fn t_8960_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8960),-4);
    }
    // 8970
    #[test]
    fn t_8970_ju_test(){
        assert_eq!(test_ju_money_number(8970),15);
    }
    #[test]
    fn t_8970_goju_test(){
        assert_eq!(test_goju_money_number(8970),-3);
    }
    #[test]
    fn t_8970_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8970),-12);
    }
    #[test]
    fn t_8970_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8970),-3);
    }
    #[test]
    fn t_8970_sen_test(){
        assert_eq!(test_sen_money_number(8970),-17);
    }
    #[test]
    fn t_8970_gosen_test(){
        assert_eq!(test_gosen_money_number(8970),-6);
    }
    #[test]
    fn t_8970_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8970),-4);
    }
    // 8980
    #[test]
    fn t_8980_ju_test(){
        assert_eq!(test_ju_money_number(8980),5);
    }
    #[test]
    fn t_8980_goju_test(){
        assert_eq!(test_goju_money_number(8980),-3);
    }
    #[test]
    fn t_8980_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8980),-12);
    }
    #[test]
    fn t_8980_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8980),-3);
    }
    #[test]
    fn t_8980_sen_test(){
        assert_eq!(test_sen_money_number(8980),-17);
    }
    #[test]
    fn t_8980_gosen_test(){
        assert_eq!(test_gosen_money_number(8980),-6);
    }
    #[test]
    fn t_8980_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8980),-4);
    }
    // 8990
    #[test]
    fn t_8990_ju_test(){
        assert_eq!(test_ju_money_number(8990),-5);
    }
    #[test]
    fn t_8990_goju_test(){
        assert_eq!(test_goju_money_number(8990),-3);
    }
    #[test]
    fn t_8990_hyaku_test(){
        assert_eq!(test_hyaku_money_number(8990),-12);
    }
    #[test]
    fn t_8990_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(8990),-3);
    }
    #[test]
    fn t_8990_sen_test(){
        assert_eq!(test_sen_money_number(8990),-17);
    }
    #[test]
    fn t_8990_gosen_test(){
        assert_eq!(test_gosen_money_number(8990),-6);
    }
    #[test]
    fn t_8990_ichiman_test(){
        assert_eq!(test_ichiman_money_number(8990),-4);
    }
    // 9000
    #[test]
    fn t_9000_ju_test(){
        assert_eq!(test_ju_money_number(9000),0);
    }
    #[test]
    fn t_9000_goju_test(){
        assert_eq!(test_goju_money_number(9000),0);
    }
    #[test]
    fn t_9000_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9000),0);
    }
    #[test]
    fn t_9000_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9000),0);
    }
    #[test]
    fn t_9000_sen_test(){
        assert_eq!(test_sen_money_number(9000),-15);
    }
    #[test]
    fn t_9000_gosen_test(){
        assert_eq!(test_gosen_money_number(9000),-5);
    }
    #[test]
    fn t_9000_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9000),-5);
    }
    // 9010
    #[test]
    fn t_9010_ju_test(){
        assert_eq!(test_ju_money_number(9010),30);
    }
    #[test]
    fn t_9010_goju_test(){
        assert_eq!(test_goju_money_number(9010),4);
    }
    #[test]
    fn t_9010_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9010),14);
    }
    #[test]
    fn t_9010_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9010),0);
    }
    #[test]
    fn t_9010_sen_test(){
        assert_eq!(test_sen_money_number(9010),-32);
    }
    #[test]
    fn t_9010_gosen_test(){
        assert_eq!(test_gosen_money_number(9010),-8);
    }
    #[test]
    fn t_9010_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9010),-2);
    }
    // 9020
    #[test]
    fn t_9020_ju_test(){
        assert_eq!(test_ju_money_number(9020),20);
    }
    #[test]
    fn t_9020_goju_test(){
        assert_eq!(test_goju_money_number(9020),4);
    }
    #[test]
    fn t_9020_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9020),14);
    }
    #[test]
    fn t_9020_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9020),0);
    }
    #[test]
    fn t_9020_sen_test(){
        assert_eq!(test_sen_money_number(9020),-32);
    }
    #[test]
    fn t_9020_gosen_test(){
        assert_eq!(test_gosen_money_number(9020),-8);
    }
    #[test]
    fn t_9020_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9020),-2);
    }
    // 9030
    #[test]
    fn t_9030_ju_test(){
        assert_eq!(test_ju_money_number(9030),10);
    }
    #[test]
    fn t_9030_goju_test(){
        assert_eq!(test_goju_money_number(9030),4);
    }
    #[test]
    fn t_9030_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9030),14);
    }
    #[test]
    fn t_9030_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9030),0);
    }
    #[test]
    fn t_9030_sen_test(){
        assert_eq!(test_sen_money_number(9030),-32);
    }
    #[test]
    fn t_9030_gosen_test(){
        assert_eq!(test_gosen_money_number(9030),-8);
    }
    #[test]
    fn t_9030_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9030),-2);
    }
    // 9040
    #[test]
    fn t_9040_ju_test(){
        assert_eq!(test_ju_money_number(9040),0);
    }
    #[test]
    fn t_9040_goju_test(){
        assert_eq!(test_goju_money_number(9040),4);
    }
    #[test]
    fn t_9040_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9040),14);
    }
    #[test]
    fn t_9040_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9040),0);
    }
    #[test]
    fn t_9040_sen_test(){
        assert_eq!(test_sen_money_number(9040),-32);
    }
    #[test]
    fn t_9040_gosen_test(){
        assert_eq!(test_gosen_money_number(9040),-8);
    }
    #[test]
    fn t_9040_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9040),-2);
    }
    // 9050
    #[test]
    fn t_9050_ju_test(){
        assert_eq!(test_ju_money_number(9050),0);
    }
    #[test]
    fn t_9050_goju_test(){
        assert_eq!(test_goju_money_number(9050),6);
    }
    #[test]
    fn t_9050_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9050),22);
    }
    #[test]
    fn t_9050_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9050),2);
    }
    #[test]
    fn t_9050_sen_test(){
        assert_eq!(test_sen_money_number(9050),-24);
    }
    #[test]
    fn t_9050_gosen_test(){
        assert_eq!(test_gosen_money_number(9050),-6);
    }
    #[test]
    fn t_9050_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9050),-4);
    }
    // 9060
    #[test]
    fn t_9060_ju_test(){
        assert_eq!(test_ju_money_number(9060),30);
    }
    #[test]
    fn t_9060_goju_test(){
        assert_eq!(test_goju_money_number(9060),-2);
    }
    #[test]
    fn t_9060_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9060),22);
    }
    #[test]
    fn t_9060_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9060),2);
    }
    #[test]
    fn t_9060_sen_test(){
        assert_eq!(test_sen_money_number(9060),-24);
    }
    #[test]
    fn t_9060_gosen_test(){
        assert_eq!(test_gosen_money_number(9060),-6);
    }
    #[test]
    fn t_9060_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9060),-4);
    }
    // 9070
    #[test]
    fn t_9070_ju_test(){
        assert_eq!(test_ju_money_number(9070),20);
    }
    #[test]
    fn t_9070_goju_test(){
        assert_eq!(test_goju_money_number(9070),-2);
    }
    #[test]
    fn t_9070_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9070),22);
    }
    #[test]
    fn t_9070_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9070),2);
    }
    #[test]
    fn t_9070_sen_test(){
        assert_eq!(test_sen_money_number(9070),-24);
    }
    #[test]
    fn t_9070_gosen_test(){
        assert_eq!(test_gosen_money_number(9070),-6);
    }
    #[test]
    fn t_9070_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9070),-4);
    }
    // 9080
    #[test]
    fn t_9080_ju_test(){
        assert_eq!(test_ju_money_number(9080),10);
    }
    #[test]
    fn t_9080_goju_test(){
        assert_eq!(test_goju_money_number(9080),-2);
    }
    #[test]
    fn t_9080_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9080),22);
    }
    #[test]
    fn t_9080_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9080),2);
    }
    #[test]
    fn t_9080_sen_test(){
        assert_eq!(test_sen_money_number(9080),-24);
    }
    #[test]
    fn t_9080_gosen_test(){
        assert_eq!(test_gosen_money_number(9080),-6);
    }
    #[test]
    fn t_9080_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9080),-4);
    }
    // 9090
    #[test]
    fn t_9090_ju_test(){
        assert_eq!(test_ju_money_number(9090),0);
    }
    #[test]
    fn t_9090_goju_test(){
        assert_eq!(test_goju_money_number(9090),-2);
    }
    #[test]
    fn t_9090_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9090),22);
    }
    #[test]
    fn t_9090_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9090),2);
    }
    #[test]
    fn t_9090_sen_test(){
        assert_eq!(test_sen_money_number(9090),-24);
    }
    #[test]
    fn t_9090_gosen_test(){
        assert_eq!(test_gosen_money_number(9090),-6);
    }
    #[test]
    fn t_9090_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9090),-4);
    }
    // 9100
    #[test]
    fn t_9100_ju_test(){
        assert_eq!(test_ju_money_number(9100),0);
    }
    #[test]
    fn t_9100_goju_test(){
        assert_eq!(test_goju_money_number(9100),0);
    }
    #[test]
    fn t_9100_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9100),25);
    }
    #[test]
    fn t_9100_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9100),1);
    }
    #[test]
    fn t_9100_sen_test(){
        assert_eq!(test_sen_money_number(9100),-24);
    }
    #[test]
    fn t_9100_gosen_test(){
        assert_eq!(test_gosen_money_number(9100),-6);
    }
    #[test]
    fn t_9100_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9100),-4);
    }
    // 9110
    #[test]
    fn t_9110_ju_test(){
        assert_eq!(test_ju_money_number(9110),30);
    }
    #[test]
    fn t_9110_goju_test(){
        assert_eq!(test_goju_money_number(9110),4);
    }
    #[test]
    fn t_9110_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9110),4);
    }
    #[test]
    fn t_9110_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9110),0);
    }
    #[test]
    fn t_9110_sen_test(){
        assert_eq!(test_sen_money_number(9110),-32);
    }
    #[test]
    fn t_9110_gosen_test(){
        assert_eq!(test_gosen_money_number(9110),-8);
    }
    #[test]
    fn t_9110_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9110),-2);
    }
    // 9120
    #[test]
    fn t_9120_ju_test(){
        assert_eq!(test_ju_money_number(9120),20);
    }
    #[test]
    fn t_9120_goju_test(){
        assert_eq!(test_goju_money_number(9120),4);
    }
    #[test]
    fn t_9120_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9120),4);
    }
    #[test]
    fn t_9120_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9120),0);
    }
    #[test]
    fn t_9120_sen_test(){
        assert_eq!(test_sen_money_number(9120),-32);
    }
    #[test]
    fn t_9120_gosen_test(){
        assert_eq!(test_gosen_money_number(9120),-8);
    }
    #[test]
    fn t_9120_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9120),-2);
    }
    // 9130
    #[test]
    fn t_9130_ju_test(){
        assert_eq!(test_ju_money_number(9130),10);
    }
    #[test]
    fn t_9130_goju_test(){
        assert_eq!(test_goju_money_number(9130),4);
    }
    #[test]
    fn t_9130_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9130),4);
    }
    #[test]
    fn t_9130_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9130),0);
    }
    #[test]
    fn t_9130_sen_test(){
        assert_eq!(test_sen_money_number(9130),-32);
    }
    #[test]
    fn t_9130_gosen_test(){
        assert_eq!(test_gosen_money_number(9130),-8);
    }
    #[test]
    fn t_9130_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9130),-2);
    }
    // 9140
    #[test]
    fn t_9140_ju_test(){
        assert_eq!(test_ju_money_number(9140),0);
    }
    #[test]
    fn t_9140_goju_test(){
        assert_eq!(test_goju_money_number(9140),4);
    }
    #[test]
    fn t_9140_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9140),4);
    }
    #[test]
    fn t_9140_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9140),0);
    }
    #[test]
    fn t_9140_sen_test(){
        assert_eq!(test_sen_money_number(9140),-32);
    }
    #[test]
    fn t_9140_gosen_test(){
        assert_eq!(test_gosen_money_number(9140),-8);
    }
    #[test]
    fn t_9140_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9140),-2);
    }
    // 9150
    #[test]
    fn t_9150_ju_test(){
        assert_eq!(test_ju_money_number(9150),0);
    }
    #[test]
    fn t_9150_goju_test(){
        assert_eq!(test_goju_money_number(9150),6);
    }
    #[test]
    fn t_9150_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9150),12);
    }
    #[test]
    fn t_9150_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9150),2);
    }
    #[test]
    fn t_9150_sen_test(){
        assert_eq!(test_sen_money_number(9150),-24);
    }
    #[test]
    fn t_9150_gosen_test(){
        assert_eq!(test_gosen_money_number(9150),-6);
    }
    #[test]
    fn t_9150_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9150),-4);
    }
    // 9160
    #[test]
    fn t_9160_ju_test(){
        assert_eq!(test_ju_money_number(9160),30);
    }
    #[test]
    fn t_9160_goju_test(){
        assert_eq!(test_goju_money_number(9160),-2);
    }
    #[test]
    fn t_9160_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9160),12);
    }
    #[test]
    fn t_9160_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9160),2);
    }
    #[test]
    fn t_9160_sen_test(){
        assert_eq!(test_sen_money_number(9160),-24);
    }
    #[test]
    fn t_9160_gosen_test(){
        assert_eq!(test_gosen_money_number(9160),-6);
    }
    #[test]
    fn t_9160_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9160),-4);
    }
    // 9170
    #[test]
    fn t_9170_ju_test(){
        assert_eq!(test_ju_money_number(9170),20);
    }
    #[test]
    fn t_9170_goju_test(){
        assert_eq!(test_goju_money_number(9170),-2);
    }
    #[test]
    fn t_9170_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9170),12);
    }
    #[test]
    fn t_9170_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9170),2);
    }
    #[test]
    fn t_9170_sen_test(){
        assert_eq!(test_sen_money_number(9170),-24);
    }
    #[test]
    fn t_9170_gosen_test(){
        assert_eq!(test_gosen_money_number(9170),-6);
    }
    #[test]
    fn t_9170_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9170),-4);
    }
    // 9180
    #[test]
    fn t_9180_ju_test(){
        assert_eq!(test_ju_money_number(9180),10);
    }
    #[test]
    fn t_9180_goju_test(){
        assert_eq!(test_goju_money_number(9180),-2);
    }
    #[test]
    fn t_9180_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9180),12);
    }
    #[test]
    fn t_9180_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9180),2);
    }
    #[test]
    fn t_9180_sen_test(){
        assert_eq!(test_sen_money_number(9180),-24);
    }
    #[test]
    fn t_9180_gosen_test(){
        assert_eq!(test_gosen_money_number(9180),-6);
    }
    #[test]
    fn t_9180_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9180),-4);
    }
    // 9190
    #[test]
    fn t_9190_ju_test(){
        assert_eq!(test_ju_money_number(9190),0);
    }
    #[test]
    fn t_9190_goju_test(){
        assert_eq!(test_goju_money_number(9190),-2);
    }
    #[test]
    fn t_9190_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9190),12);
    }
    #[test]
    fn t_9190_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9190),2);
    }
    #[test]
    fn t_9190_sen_test(){
        assert_eq!(test_sen_money_number(9190),-24);
    }
    #[test]
    fn t_9190_gosen_test(){
        assert_eq!(test_gosen_money_number(9190),-6);
    }
    #[test]
    fn t_9190_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9190),-4);
    }
    // 9200
    #[test]
    fn t_9200_ju_test(){
        assert_eq!(test_ju_money_number(9200),0);
    }
    #[test]
    fn t_9200_goju_test(){
        assert_eq!(test_goju_money_number(9200),0);
    }
    #[test]
    fn t_9200_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9200),15);
    }
    #[test]
    fn t_9200_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9200),1);
    }
    #[test]
    fn t_9200_sen_test(){
        assert_eq!(test_sen_money_number(9200),-24);
    }
    #[test]
    fn t_9200_gosen_test(){
        assert_eq!(test_gosen_money_number(9200),-6);
    }
    #[test]
    fn t_9200_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9200),-4);
    }
    // 9210
    #[test]
    fn t_9210_ju_test(){
        assert_eq!(test_ju_money_number(9210),30);
    }
    #[test]
    fn t_9210_goju_test(){
        assert_eq!(test_goju_money_number(9210),4);
    }
    #[test]
    fn t_9210_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9210),-6);
    }
    #[test]
    fn t_9210_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9210),0);
    }
    #[test]
    fn t_9210_sen_test(){
        assert_eq!(test_sen_money_number(9210),-32);
    }
    #[test]
    fn t_9210_gosen_test(){
        assert_eq!(test_gosen_money_number(9210),-8);
    }
    #[test]
    fn t_9210_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9210),-2);
    }
    // 9220
    #[test]
    fn t_9220_ju_test(){
        assert_eq!(test_ju_money_number(9220),20);
    }
    #[test]
    fn t_9220_goju_test(){
        assert_eq!(test_goju_money_number(9220),4);
    }
    #[test]
    fn t_9220_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9220),-6);
    }
    #[test]
    fn t_9220_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9220),0);
    }
    #[test]
    fn t_9220_sen_test(){
        assert_eq!(test_sen_money_number(9220),-32);
    }
    #[test]
    fn t_9220_gosen_test(){
        assert_eq!(test_gosen_money_number(9220),-8);
    }
    #[test]
    fn t_9220_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9220),-2);
    }
    // 9230
    #[test]
    fn t_9230_ju_test(){
        assert_eq!(test_ju_money_number(9230),10);
    }
    #[test]
    fn t_9230_goju_test(){
        assert_eq!(test_goju_money_number(9230),4);
    }
    #[test]
    fn t_9230_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9230),-6);
    }
    #[test]
    fn t_9230_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9230),0);
    }
    #[test]
    fn t_9230_sen_test(){
        assert_eq!(test_sen_money_number(9230),-32);
    }
    #[test]
    fn t_9230_gosen_test(){
        assert_eq!(test_gosen_money_number(9230),-8);
    }
    #[test]
    fn t_9230_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9230),-2);
    }
    // 9240
    #[test]
    fn t_9240_ju_test(){
        assert_eq!(test_ju_money_number(9240),0);
    }
    #[test]
    fn t_9240_goju_test(){
        assert_eq!(test_goju_money_number(9240),4);
    }
    #[test]
    fn t_9240_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9240),-6);
    }
    #[test]
    fn t_9240_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9240),0);
    }
    #[test]
    fn t_9240_sen_test(){
        assert_eq!(test_sen_money_number(9240),-32);
    }
    #[test]
    fn t_9240_gosen_test(){
        assert_eq!(test_gosen_money_number(9240),-8);
    }
    #[test]
    fn t_9240_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9240),-2);
    }
    // 9250
    #[test]
    fn t_9250_ju_test(){
        assert_eq!(test_ju_money_number(9250),0);
    }
    #[test]
    fn t_9250_goju_test(){
        assert_eq!(test_goju_money_number(9250),6);
    }
    #[test]
    fn t_9250_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9250),2);
    }
    #[test]
    fn t_9250_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9250),2);
    }
    #[test]
    fn t_9250_sen_test(){
        assert_eq!(test_sen_money_number(9250),-24);
    }
    #[test]
    fn t_9250_gosen_test(){
        assert_eq!(test_gosen_money_number(9250),-6);
    }
    #[test]
    fn t_9250_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9250),-4);
    }
    // 9260
    #[test]
    fn t_9260_ju_test(){
        assert_eq!(test_ju_money_number(9260),30);
    }
    #[test]
    fn t_9260_goju_test(){
        assert_eq!(test_goju_money_number(9260),-2);
    }
    #[test]
    fn t_9260_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9260),2);
    }
    #[test]
    fn t_9260_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9260),2);
    }
    #[test]
    fn t_9260_sen_test(){
        assert_eq!(test_sen_money_number(9260),-24);
    }
    #[test]
    fn t_9260_gosen_test(){
        assert_eq!(test_gosen_money_number(9260),-6);
    }
    #[test]
    fn t_9260_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9260),-4);
    }
    // 9270
    #[test]
    fn t_9270_ju_test(){
        assert_eq!(test_ju_money_number(9270),20);
    }
    #[test]
    fn t_9270_goju_test(){
        assert_eq!(test_goju_money_number(9270),-2);
    }
    #[test]
    fn t_9270_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9270),2);
    }
    #[test]
    fn t_9270_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9270),2);
    }
    #[test]
    fn t_9270_sen_test(){
        assert_eq!(test_sen_money_number(9270),-24);
    }
    #[test]
    fn t_9270_gosen_test(){
        assert_eq!(test_gosen_money_number(9270),-6);
    }
    #[test]
    fn t_9270_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9270),-4);
    }
    // 9280
    #[test]
    fn t_9280_ju_test(){
        assert_eq!(test_ju_money_number(9280),10);
    }
    #[test]
    fn t_9280_goju_test(){
        assert_eq!(test_goju_money_number(9280),-2);
    }
    #[test]
    fn t_9280_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9280),2);
    }
    #[test]
    fn t_9280_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9280),2);
    }
    #[test]
    fn t_9280_sen_test(){
        assert_eq!(test_sen_money_number(9280),-24);
    }
    #[test]
    fn t_9280_gosen_test(){
        assert_eq!(test_gosen_money_number(9280),-6);
    }
    #[test]
    fn t_9280_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9280),-4);
    }
    // 9290
    #[test]
    fn t_9290_ju_test(){
        assert_eq!(test_ju_money_number(9290),0);
    }
    #[test]
    fn t_9290_goju_test(){
        assert_eq!(test_goju_money_number(9290),-2);
    }
    #[test]
    fn t_9290_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9290),2);
    }
    #[test]
    fn t_9290_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9290),2);
    }
    #[test]
    fn t_9290_sen_test(){
        assert_eq!(test_sen_money_number(9290),-24);
    }
    #[test]
    fn t_9290_gosen_test(){
        assert_eq!(test_gosen_money_number(9290),-6);
    }
    #[test]
    fn t_9290_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9290),-4);
    }
    // 9300
    #[test]
    fn t_9300_ju_test(){
        assert_eq!(test_ju_money_number(9300),0);
    }
    #[test]
    fn t_9300_goju_test(){
        assert_eq!(test_goju_money_number(9300),0);
    }
    #[test]
    fn t_9300_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9300),5);
    }
    #[test]
    fn t_9300_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9300),1);
    }
    #[test]
    fn t_9300_sen_test(){
        assert_eq!(test_sen_money_number(9300),-24);
    }
    #[test]
    fn t_9300_gosen_test(){
        assert_eq!(test_gosen_money_number(9300),-6);
    }
    #[test]
    fn t_9300_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9300),-4);
    }
    // 9310
    #[test]
    fn t_9310_ju_test(){
        assert_eq!(test_ju_money_number(9310),30);
    }
    #[test]
    fn t_9310_goju_test(){
        assert_eq!(test_goju_money_number(9310),4);
    }
    #[test]
    fn t_9310_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9310),-16);
    }
    #[test]
    fn t_9310_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9310),0);
    }
    #[test]
    fn t_9310_sen_test(){
        assert_eq!(test_sen_money_number(9310),-32);
    }
    #[test]
    fn t_9310_gosen_test(){
        assert_eq!(test_gosen_money_number(9310),-8);
    }
    #[test]
    fn t_9310_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9310),-2);
    }
    // 9320
    #[test]
    fn t_9320_ju_test(){
        assert_eq!(test_ju_money_number(9320),20);
    }
    #[test]
    fn t_9320_goju_test(){
        assert_eq!(test_goju_money_number(9320),4);
    }
    #[test]
    fn t_9320_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9320),-16);
    }
    #[test]
    fn t_9320_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9320),0);
    }
    #[test]
    fn t_9320_sen_test(){
        assert_eq!(test_sen_money_number(9320),-32);
    }
    #[test]
    fn t_9320_gosen_test(){
        assert_eq!(test_gosen_money_number(9320),-8);
    }
    #[test]
    fn t_9320_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9320),-2);
    }
    // 9330
    #[test]
    fn t_9330_ju_test(){
        assert_eq!(test_ju_money_number(9330),10);
    }
    #[test]
    fn t_9330_goju_test(){
        assert_eq!(test_goju_money_number(9330),4);
    }
    #[test]
    fn t_9330_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9330),-16);
    }
    #[test]
    fn t_9330_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9330),0);
    }
    #[test]
    fn t_9330_sen_test(){
        assert_eq!(test_sen_money_number(9330),-32);
    }
    #[test]
    fn t_9330_gosen_test(){
        assert_eq!(test_gosen_money_number(9330),-8);
    }
    #[test]
    fn t_9330_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9330),-2);
    }
    // 9340
    #[test]
    fn t_9340_ju_test(){
        assert_eq!(test_ju_money_number(9340),0);
    }
    #[test]
    fn t_9340_goju_test(){
        assert_eq!(test_goju_money_number(9340),4);
    }
    #[test]
    fn t_9340_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9340),-16);
    }
    #[test]
    fn t_9340_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9340),0);
    }
    #[test]
    fn t_9340_sen_test(){
        assert_eq!(test_sen_money_number(9340),-32);
    }
    #[test]
    fn t_9340_gosen_test(){
        assert_eq!(test_gosen_money_number(9340),-8);
    }
    #[test]
    fn t_9340_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9340),-2);
    }
    // 9350
    #[test]
    fn t_9350_ju_test(){
        assert_eq!(test_ju_money_number(9350),0);
    }
    #[test]
    fn t_9350_goju_test(){
        assert_eq!(test_goju_money_number(9350),6);
    }
    #[test]
    fn t_9350_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9350),-8);
    }
    #[test]
    fn t_9350_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9350),2);
    }
    #[test]
    fn t_9350_sen_test(){
        assert_eq!(test_sen_money_number(9350),-24);
    }
    #[test]
    fn t_9350_gosen_test(){
        assert_eq!(test_gosen_money_number(9350),-6);
    }
    #[test]
    fn t_9350_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9350),-4);
    }
    // 9360
    #[test]
    fn t_9360_ju_test(){
        assert_eq!(test_ju_money_number(9360),30);
    }
    #[test]
    fn t_9360_goju_test(){
        assert_eq!(test_goju_money_number(9360),-2);
    }
    #[test]
    fn t_9360_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9360),-8);
    }
    #[test]
    fn t_9360_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9360),2);
    }
    #[test]
    fn t_9360_sen_test(){
        assert_eq!(test_sen_money_number(9360),-24);
    }
    #[test]
    fn t_9360_gosen_test(){
        assert_eq!(test_gosen_money_number(9360),-6);
    }
    #[test]
    fn t_9360_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9360),-4);
    }
    // 9370
    #[test]
    fn t_9370_ju_test(){
        assert_eq!(test_ju_money_number(9370),20);
    }
    #[test]
    fn t_9370_goju_test(){
        assert_eq!(test_goju_money_number(9370),-2);
    }
    #[test]
    fn t_9370_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9370),-8);
    }
    #[test]
    fn t_9370_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9370),2);
    }
    #[test]
    fn t_9370_sen_test(){
        assert_eq!(test_sen_money_number(9370),-24);
    }
    #[test]
    fn t_9370_gosen_test(){
        assert_eq!(test_gosen_money_number(9370),-6);
    }
    #[test]
    fn t_9370_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9370),-4);
    }
    // 9380
    #[test]
    fn t_9380_ju_test(){
        assert_eq!(test_ju_money_number(9380),10);
    }
    #[test]
    fn t_9380_goju_test(){
        assert_eq!(test_goju_money_number(9380),-2);
    }
    #[test]
    fn t_9380_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9380),-8);
    }
    #[test]
    fn t_9380_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9380),2);
    }
    #[test]
    fn t_9380_sen_test(){
        assert_eq!(test_sen_money_number(9380),-24);
    }
    #[test]
    fn t_9380_gosen_test(){
        assert_eq!(test_gosen_money_number(9380),-6);
    }
    #[test]
    fn t_9380_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9380),-4);
    }
    // 9390
    #[test]
    fn t_9390_ju_test(){
        assert_eq!(test_ju_money_number(9390),0);
    }
    #[test]
    fn t_9390_goju_test(){
        assert_eq!(test_goju_money_number(9390),-2);
    }
    #[test]
    fn t_9390_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9390),-8);
    }
    #[test]
    fn t_9390_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9390),2);
    }
    #[test]
    fn t_9390_sen_test(){
        assert_eq!(test_sen_money_number(9390),-24);
    }
    #[test]
    fn t_9390_gosen_test(){
        assert_eq!(test_gosen_money_number(9390),-6);
    }
    #[test]
    fn t_9390_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9390),-4);
    }
    // 9400
    #[test]
    fn t_9400_ju_test(){
        assert_eq!(test_ju_money_number(9400),0);
    }
    #[test]
    fn t_9400_goju_test(){
        assert_eq!(test_goju_money_number(9400),0);
    }
    #[test]
    fn t_9400_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9400),-5);
    }
    #[test]
    fn t_9400_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9400),1);
    }
    #[test]
    fn t_9400_sen_test(){
        assert_eq!(test_sen_money_number(9400),-24);
    }
    #[test]
    fn t_9400_gosen_test(){
        assert_eq!(test_gosen_money_number(9400),-6);
    }
    #[test]
    fn t_9400_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9400),-4);
    }
    // 9410
    #[test]
    fn t_9410_ju_test(){
        assert_eq!(test_ju_money_number(9410),30);
    }
    #[test]
    fn t_9410_goju_test(){
        assert_eq!(test_goju_money_number(9410),4);
    }
    #[test]
    fn t_9410_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9410),-16);
    }
    #[test]
    fn t_9410_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9410),2);
    }
    #[test]
    fn t_9410_sen_test(){
        assert_eq!(test_sen_money_number(9410),-24);
    }
    #[test]
    fn t_9410_gosen_test(){
        assert_eq!(test_gosen_money_number(9410),-6);
    }
    #[test]
    fn t_9410_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9410),-4);
    }
    // 9420
    #[test]
    fn t_9420_ju_test(){
        assert_eq!(test_ju_money_number(9420),20);
    }
    #[test]
    fn t_9420_goju_test(){
        assert_eq!(test_goju_money_number(9420),4);
    }
    #[test]
    fn t_9420_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9420),-16);
    }
    #[test]
    fn t_9420_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9420),2);
    }
    #[test]
    fn t_9420_sen_test(){
        assert_eq!(test_sen_money_number(9420),-24);
    }
    #[test]
    fn t_9420_gosen_test(){
        assert_eq!(test_gosen_money_number(9420),-6);
    }
    #[test]
    fn t_9420_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9420),-4);
    }
    // 9430
    #[test]
    fn t_9430_ju_test(){
        assert_eq!(test_ju_money_number(9430),10);
    }
    #[test]
    fn t_9430_goju_test(){
        assert_eq!(test_goju_money_number(9430),4);
    }
    #[test]
    fn t_9430_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9430),-16);
    }
    #[test]
    fn t_9430_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9430),2);
    }
    #[test]
    fn t_9430_sen_test(){
        assert_eq!(test_sen_money_number(9430),-24);
    }
    #[test]
    fn t_9430_gosen_test(){
        assert_eq!(test_gosen_money_number(9430),-6);
    }
    #[test]
    fn t_9430_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9430),-4);
    }
    // 9440
    #[test]
    fn t_9440_ju_test(){
        assert_eq!(test_ju_money_number(9440),0);
    }
    #[test]
    fn t_9440_goju_test(){
        assert_eq!(test_goju_money_number(9440),4);
    }
    #[test]
    fn t_9440_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9440),-16);
    }
    #[test]
    fn t_9440_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9440),2);
    }
    #[test]
    fn t_9440_sen_test(){
        assert_eq!(test_sen_money_number(9440),-24);
    }
    #[test]
    fn t_9440_gosen_test(){
        assert_eq!(test_gosen_money_number(9440),-6);
    }
    #[test]
    fn t_9440_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9440),-4);
    }
    // 9450
    #[test]
    fn t_9450_ju_test(){
        assert_eq!(test_ju_money_number(9450),0);
    }
    #[test]
    fn t_9450_goju_test(){
        assert_eq!(test_goju_money_number(9450),4);
    }
    #[test]
    fn t_9450_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9450),-12);
    }
    #[test]
    fn t_9450_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9450),1);
    }
    #[test]
    fn t_9450_sen_test(){
        assert_eq!(test_sen_money_number(9450),-24);
    }
    #[test]
    fn t_9450_gosen_test(){
        assert_eq!(test_gosen_money_number(9450),-6);
    }
    #[test]
    fn t_9450_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9450),-4);
    }
    // 9460
    #[test]
    fn t_9460_ju_test(){
        assert_eq!(test_ju_money_number(9460),25);
    }
    #[test]
    fn t_9460_goju_test(){
        assert_eq!(test_goju_money_number(9460),-3);
    }
    #[test]
    fn t_9460_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9460),-12);
    }
    #[test]
    fn t_9460_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9460),1);
    }
    #[test]
    fn t_9460_sen_test(){
        assert_eq!(test_sen_money_number(9460),-24);
    }
    #[test]
    fn t_9460_gosen_test(){
        assert_eq!(test_gosen_money_number(9460),-6);
    }
    #[test]
    fn t_9460_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9460),-4);
    }
    // 9470
    #[test]
    fn t_9470_ju_test(){
        assert_eq!(test_ju_money_number(9470),15);
    }
    #[test]
    fn t_9470_goju_test(){
        assert_eq!(test_goju_money_number(9470),-3);
    }
    #[test]
    fn t_9470_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9470),-12);
    }
    #[test]
    fn t_9470_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9470),1);
    }
    #[test]
    fn t_9470_sen_test(){
        assert_eq!(test_sen_money_number(9470),-24);
    }
    #[test]
    fn t_9470_gosen_test(){
        assert_eq!(test_gosen_money_number(9470),-6);
    }
    #[test]
    fn t_9470_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9470),-4);
    }
    // 9480
    #[test]
    fn t_9480_ju_test(){
        assert_eq!(test_ju_money_number(9480),5);
    }
    #[test]
    fn t_9480_goju_test(){
        assert_eq!(test_goju_money_number(9480),-3);
    }
    #[test]
    fn t_9480_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9480),-12);
    }
    #[test]
    fn t_9480_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9480),1);
    }
    #[test]
    fn t_9480_sen_test(){
        assert_eq!(test_sen_money_number(9480),-24);
    }
    #[test]
    fn t_9480_gosen_test(){
        assert_eq!(test_gosen_money_number(9480),-6);
    }
    #[test]
    fn t_9480_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9480),-4);
    }
    // 9490
    #[test]
    fn t_9490_ju_test(){
        assert_eq!(test_ju_money_number(9490),-5);
    }
    #[test]
    fn t_9490_goju_test(){
        assert_eq!(test_goju_money_number(9490),-3);
    }
    #[test]
    fn t_9490_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9490),-12);
    }
    #[test]
    fn t_9490_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9490),1);
    }
    #[test]
    fn t_9490_sen_test(){
        assert_eq!(test_sen_money_number(9490),-24);
    }
    #[test]
    fn t_9490_gosen_test(){
        assert_eq!(test_gosen_money_number(9490),-6);
    }
    #[test]
    fn t_9490_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9490),-4);
    }
    // 9500
    #[test]
    fn t_9500_ju_test(){
        assert_eq!(test_ju_money_number(9500),0);
    }
    #[test]
    fn t_9500_goju_test(){
        assert_eq!(test_goju_money_number(9500),0);
    }
    #[test]
    fn t_9500_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9500),0);
    }
    #[test]
    fn t_9500_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9500),0);
    }
    #[test]
    fn t_9500_sen_test(){
        assert_eq!(test_sen_money_number(9500),-20);
    }
    #[test]
    fn t_9500_gosen_test(){
        assert_eq!(test_gosen_money_number(9500),-5);
    }
    #[test]
    fn t_9500_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9500),-5);
    }
    // 9510
    #[test]
    fn t_9510_ju_test(){
        assert_eq!(test_ju_money_number(9510),30);
    }
    #[test]
    fn t_9510_goju_test(){
        assert_eq!(test_goju_money_number(9510),4);
    }
    #[test]
    fn t_9510_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9510),14);
    }
    #[test]
    fn t_9510_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9510),-6);
    }
    #[test]
    fn t_9510_sen_test(){
        assert_eq!(test_sen_money_number(9510),-24);
    }
    #[test]
    fn t_9510_gosen_test(){
        assert_eq!(test_gosen_money_number(9510),-6);
    }
    #[test]
    fn t_9510_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9510),-4);
    }
    // 9520
    #[test]
    fn t_9520_ju_test(){
        assert_eq!(test_ju_money_number(9520),20);
    }
    #[test]
    fn t_9520_goju_test(){
        assert_eq!(test_goju_money_number(9520),4);
    }
    #[test]
    fn t_9520_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9520),14);
    }
    #[test]
    fn t_9520_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9520),-6);
    }
    #[test]
    fn t_9520_sen_test(){
        assert_eq!(test_sen_money_number(9520),-24);
    }
    #[test]
    fn t_9520_gosen_test(){
        assert_eq!(test_gosen_money_number(9520),-6);
    }
    #[test]
    fn t_9520_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9520),-4);
    }
    // 9530
    #[test]
    fn t_9530_ju_test(){
        assert_eq!(test_ju_money_number(9530),10);
    }
    #[test]
    fn t_9530_goju_test(){
        assert_eq!(test_goju_money_number(9530),4);
    }
    #[test]
    fn t_9530_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9530),14);
    }
    #[test]
    fn t_9530_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9530),-6);
    }
    #[test]
    fn t_9530_sen_test(){
        assert_eq!(test_sen_money_number(9530),-24);
    }
    #[test]
    fn t_9530_gosen_test(){
        assert_eq!(test_gosen_money_number(9530),-6);
    }
    #[test]
    fn t_9530_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9530),-4);
    }
    // 9540
    #[test]
    fn t_9540_ju_test(){
        assert_eq!(test_ju_money_number(9540),0);
    }
    #[test]
    fn t_9540_goju_test(){
        assert_eq!(test_goju_money_number(9540),4);
    }
    #[test]
    fn t_9540_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9540),14);
    }
    #[test]
    fn t_9540_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9540),-6);
    }
    #[test]
    fn t_9540_sen_test(){
        assert_eq!(test_sen_money_number(9540),-24);
    }
    #[test]
    fn t_9540_gosen_test(){
        assert_eq!(test_gosen_money_number(9540),-6);
    }
    #[test]
    fn t_9540_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9540),-4);
    }
    // 9550
    #[test]
    fn t_9550_ju_test(){
        assert_eq!(test_ju_money_number(9550),0);
    }
    #[test]
    fn t_9550_goju_test(){
        assert_eq!(test_goju_money_number(9550),4);
    }
    #[test]
    fn t_9550_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9550),13);
    }
    #[test]
    fn t_9550_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9550),-6);
    }
    #[test]
    fn t_9550_sen_test(){
        assert_eq!(test_sen_money_number(9550),-24);
    }
    #[test]
    fn t_9550_gosen_test(){
        assert_eq!(test_gosen_money_number(9550),-6);
    }
    #[test]
    fn t_9550_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9550),-4);
    }
    // 9560
    #[test]
    fn t_9560_ju_test(){
        assert_eq!(test_ju_money_number(9560),25);
    }
    #[test]
    fn t_9560_goju_test(){
        assert_eq!(test_goju_money_number(9560),-3);
    }
    #[test]
    fn t_9560_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9560),13);
    }
    #[test]
    fn t_9560_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9560),-6);
    }
    #[test]
    fn t_9560_sen_test(){
        assert_eq!(test_sen_money_number(9560),-24);
    }
    #[test]
    fn t_9560_gosen_test(){
        assert_eq!(test_gosen_money_number(9560),-6);
    }
    #[test]
    fn t_9560_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9560),-4);
    }
    // 9570
    #[test]
    fn t_9570_ju_test(){
        assert_eq!(test_ju_money_number(9570),15);
    }
    #[test]
    fn t_9570_goju_test(){
        assert_eq!(test_goju_money_number(9570),-3);
    }
    #[test]
    fn t_9570_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9570),13);
    }
    #[test]
    fn t_9570_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9570),-6);
    }
    #[test]
    fn t_9570_sen_test(){
        assert_eq!(test_sen_money_number(9570),-24);
    }
    #[test]
    fn t_9570_gosen_test(){
        assert_eq!(test_gosen_money_number(9570),-6);
    }
    #[test]
    fn t_9570_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9570),-4);
    }
    // 9580
    #[test]
    fn t_9580_ju_test(){
        assert_eq!(test_ju_money_number(9580),5);
    }
    #[test]
    fn t_9580_goju_test(){
        assert_eq!(test_goju_money_number(9580),-3);
    }
    #[test]
    fn t_9580_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9580),13);
    }
    #[test]
    fn t_9580_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9580),-6);
    }
    #[test]
    fn t_9580_sen_test(){
        assert_eq!(test_sen_money_number(9580),-24);
    }
    #[test]
    fn t_9580_gosen_test(){
        assert_eq!(test_gosen_money_number(9580),-6);
    }
    #[test]
    fn t_9580_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9580),-4);
    }
    // 9590
    #[test]
    fn t_9590_ju_test(){
        assert_eq!(test_ju_money_number(9590),-5);
    }
    #[test]
    fn t_9590_goju_test(){
        assert_eq!(test_goju_money_number(9590),-3);
    }
    #[test]
    fn t_9590_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9590),13);
    }
    #[test]
    fn t_9590_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9590),-6);
    }
    #[test]
    fn t_9590_sen_test(){
        assert_eq!(test_sen_money_number(9590),-24);
    }
    #[test]
    fn t_9590_gosen_test(){
        assert_eq!(test_gosen_money_number(9590),-6);
    }
    #[test]
    fn t_9590_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9590),-4);
    }
    // 9600
    #[test]
    fn t_9600_ju_test(){
        assert_eq!(test_ju_money_number(9600),0);
    }
    #[test]
    fn t_9600_goju_test(){
        assert_eq!(test_goju_money_number(9600),0);
    }
    #[test]
    fn t_9600_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9600),15);
    }
    #[test]
    fn t_9600_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9600),-5);
    }
    #[test]
    fn t_9600_sen_test(){
        assert_eq!(test_sen_money_number(9600),-20);
    }
    #[test]
    fn t_9600_gosen_test(){
        assert_eq!(test_gosen_money_number(9600),-5);
    }
    #[test]
    fn t_9600_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9600),-5);
    }
    // 9610
    #[test]
    fn t_9610_ju_test(){
        assert_eq!(test_ju_money_number(9610),30);
    }
    #[test]
    fn t_9610_goju_test(){
        assert_eq!(test_goju_money_number(9610),4);
    }
    #[test]
    fn t_9610_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9610),4);
    }
    #[test]
    fn t_9610_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9610),-6);
    }
    #[test]
    fn t_9610_sen_test(){
        assert_eq!(test_sen_money_number(9610),-24);
    }
    #[test]
    fn t_9610_gosen_test(){
        assert_eq!(test_gosen_money_number(9610),-6);
    }
    #[test]
    fn t_9610_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9610),-4);
    }
    // 9620
    #[test]
    fn t_9620_ju_test(){
        assert_eq!(test_ju_money_number(9620),20);
    }
    #[test]
    fn t_9620_goju_test(){
        assert_eq!(test_goju_money_number(9620),4);
    }
    #[test]
    fn t_9620_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9620),4);
    }
    #[test]
    fn t_9620_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9620),-6);
    }
    #[test]
    fn t_9620_sen_test(){
        assert_eq!(test_sen_money_number(9620),-24);
    }
    #[test]
    fn t_9620_gosen_test(){
        assert_eq!(test_gosen_money_number(9620),-6);
    }
    #[test]
    fn t_9620_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9620),-4);
    }
    // 9630
    #[test]
    fn t_9630_ju_test(){
        assert_eq!(test_ju_money_number(9630),10);
    }
    #[test]
    fn t_9630_goju_test(){
        assert_eq!(test_goju_money_number(9630),4);
    }
    #[test]
    fn t_9630_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9630),4);
    }
    #[test]
    fn t_9630_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9630),-6);
    }
    #[test]
    fn t_9630_sen_test(){
        assert_eq!(test_sen_money_number(9630),-24);
    }
    #[test]
    fn t_9630_gosen_test(){
        assert_eq!(test_gosen_money_number(9630),-6);
    }
    #[test]
    fn t_9630_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9630),-4);
    }
    // 9640
    #[test]
    fn t_9640_ju_test(){
        assert_eq!(test_ju_money_number(9640),0);
    }
    #[test]
    fn t_9640_goju_test(){
        assert_eq!(test_goju_money_number(9640),4);
    }
    #[test]
    fn t_9640_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9640),4);
    }
    #[test]
    fn t_9640_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9640),-6);
    }
    #[test]
    fn t_9640_sen_test(){
        assert_eq!(test_sen_money_number(9640),-24);
    }
    #[test]
    fn t_9640_gosen_test(){
        assert_eq!(test_gosen_money_number(9640),-6);
    }
    #[test]
    fn t_9640_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9640),-4);
    }
    // 9650
    #[test]
    fn t_9650_ju_test(){
        assert_eq!(test_ju_money_number(9650),0);
    }
    #[test]
    fn t_9650_goju_test(){
        assert_eq!(test_goju_money_number(9650),4);
    }
    #[test]
    fn t_9650_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9650),3);
    }
    #[test]
    fn t_9650_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9650),-6);
    }
    #[test]
    fn t_9650_sen_test(){
        assert_eq!(test_sen_money_number(9650),-24);
    }
    #[test]
    fn t_9650_gosen_test(){
        assert_eq!(test_gosen_money_number(9650),-6);
    }
    #[test]
    fn t_9650_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9650),-4);
    }
    // 9660
    #[test]
    fn t_9660_ju_test(){
        assert_eq!(test_ju_money_number(9660),25);
    }
    #[test]
    fn t_9660_goju_test(){
        assert_eq!(test_goju_money_number(9660),-3);
    }
    #[test]
    fn t_9660_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9660),3);
    }
    #[test]
    fn t_9660_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9660),-6);
    }
    #[test]
    fn t_9660_sen_test(){
        assert_eq!(test_sen_money_number(9660),-24);
    }
    #[test]
    fn t_9660_gosen_test(){
        assert_eq!(test_gosen_money_number(9660),-6);
    }
    #[test]
    fn t_9660_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9660),-4);
    }
    // 9670
    #[test]
    fn t_9670_ju_test(){
        assert_eq!(test_ju_money_number(9670),15);
    }
    #[test]
    fn t_9670_goju_test(){
        assert_eq!(test_goju_money_number(9670),-3);
    }
    #[test]
    fn t_9670_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9670),3);
    }
    #[test]
    fn t_9670_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9670),-6);
    }
    #[test]
    fn t_9670_sen_test(){
        assert_eq!(test_sen_money_number(9670),-24);
    }
    #[test]
    fn t_9670_gosen_test(){
        assert_eq!(test_gosen_money_number(9670),-6);
    }
    #[test]
    fn t_9670_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9670),-4);
    }
    // 9680
    #[test]
    fn t_9680_ju_test(){
        assert_eq!(test_ju_money_number(9680),5);
    }
    #[test]
    fn t_9680_goju_test(){
        assert_eq!(test_goju_money_number(9680),-3);
    }
    #[test]
    fn t_9680_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9680),3);
    }
    #[test]
    fn t_9680_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9680),-6);
    }
    #[test]
    fn t_9680_sen_test(){
        assert_eq!(test_sen_money_number(9680),-24);
    }
    #[test]
    fn t_9680_gosen_test(){
        assert_eq!(test_gosen_money_number(9680),-6);
    }
    #[test]
    fn t_9680_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9680),-4);
    }
    // 9690
    #[test]
    fn t_9690_ju_test(){
        assert_eq!(test_ju_money_number(9690),-5);
    }
    #[test]
    fn t_9690_goju_test(){
        assert_eq!(test_goju_money_number(9690),-3);
    }
    #[test]
    fn t_9690_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9690),3);
    }
    #[test]
    fn t_9690_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9690),-6);
    }
    #[test]
    fn t_9690_sen_test(){
        assert_eq!(test_sen_money_number(9690),-24);
    }
    #[test]
    fn t_9690_gosen_test(){
        assert_eq!(test_gosen_money_number(9690),-6);
    }
    #[test]
    fn t_9690_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9690),-4);
    }
    // 9700
    #[test]
    fn t_9700_ju_test(){
        assert_eq!(test_ju_money_number(9700),0);
    }
    #[test]
    fn t_9700_goju_test(){
        assert_eq!(test_goju_money_number(9700),0);
    }
    #[test]
    fn t_9700_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9700),5);
    }
    #[test]
    fn t_9700_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9700),-5);
    }
    #[test]
    fn t_9700_sen_test(){
        assert_eq!(test_sen_money_number(9700),-20);
    }
    #[test]
    fn t_9700_gosen_test(){
        assert_eq!(test_gosen_money_number(9700),-5);
    }
    #[test]
    fn t_9700_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9700),-5);
    }
    // 9710
    #[test]
    fn t_9710_ju_test(){
        assert_eq!(test_ju_money_number(9710),30);
    }
    #[test]
    fn t_9710_goju_test(){
        assert_eq!(test_goju_money_number(9710),4);
    }
    #[test]
    fn t_9710_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9710),-6);
    }
    #[test]
    fn t_9710_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9710),-6);
    }
    #[test]
    fn t_9710_sen_test(){
        assert_eq!(test_sen_money_number(9710),-24);
    }
    #[test]
    fn t_9710_gosen_test(){
        assert_eq!(test_gosen_money_number(9710),-6);
    }
    #[test]
    fn t_9710_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9710),-4);
    }
    // 9720
    #[test]
    fn t_9720_ju_test(){
        assert_eq!(test_ju_money_number(9720),20);
    }
    #[test]
    fn t_9720_goju_test(){
        assert_eq!(test_goju_money_number(9720),4);
    }
    #[test]
    fn t_9720_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9720),-6);
    }
    #[test]
    fn t_9720_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9720),-6);
    }
    #[test]
    fn t_9720_sen_test(){
        assert_eq!(test_sen_money_number(9720),-24);
    }
    #[test]
    fn t_9720_gosen_test(){
        assert_eq!(test_gosen_money_number(9720),-6);
    }
    #[test]
    fn t_9720_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9720),-4);
    }
    // 9730
    #[test]
    fn t_9730_ju_test(){
        assert_eq!(test_ju_money_number(9730),10);
    }
    #[test]
    fn t_9730_goju_test(){
        assert_eq!(test_goju_money_number(9730),4);
    }
    #[test]
    fn t_9730_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9730),-6);
    }
    #[test]
    fn t_9730_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9730),-6);
    }
    #[test]
    fn t_9730_sen_test(){
        assert_eq!(test_sen_money_number(9730),-24);
    }
    #[test]
    fn t_9730_gosen_test(){
        assert_eq!(test_gosen_money_number(9730),-6);
    }
    #[test]
    fn t_9730_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9730),-4);
    }
    // 9740
    #[test]
    fn t_9740_ju_test(){
        assert_eq!(test_ju_money_number(9740),0);
    }
    #[test]
    fn t_9740_goju_test(){
        assert_eq!(test_goju_money_number(9740),4);
    }
    #[test]
    fn t_9740_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9740),-6);
    }
    #[test]
    fn t_9740_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9740),-6);
    }
    #[test]
    fn t_9740_sen_test(){
        assert_eq!(test_sen_money_number(9740),-24);
    }
    #[test]
    fn t_9740_gosen_test(){
        assert_eq!(test_gosen_money_number(9740),-6);
    }
    #[test]
    fn t_9740_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9740),-4);
    }
    // 9750
    #[test]
    fn t_9750_ju_test(){
        assert_eq!(test_ju_money_number(9750),0);
    }
    #[test]
    fn t_9750_goju_test(){
        assert_eq!(test_goju_money_number(9750),4);
    }
    #[test]
    fn t_9750_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9750),-7);
    }
    #[test]
    fn t_9750_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9750),-6);
    }
    #[test]
    fn t_9750_sen_test(){
        assert_eq!(test_sen_money_number(9750),-24);
    }
    #[test]
    fn t_9750_gosen_test(){
        assert_eq!(test_gosen_money_number(9750),-6);
    }
    #[test]
    fn t_9750_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9750),-4);
    }
    // 9760
    #[test]
    fn t_9760_ju_test(){
        assert_eq!(test_ju_money_number(9760),25);
    }
    #[test]
    fn t_9760_goju_test(){
        assert_eq!(test_goju_money_number(9760),-3);
    }
    #[test]
    fn t_9760_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9760),-7);
    }
    #[test]
    fn t_9760_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9760),-6);
    }
    #[test]
    fn t_9760_sen_test(){
        assert_eq!(test_sen_money_number(9760),-24);
    }
    #[test]
    fn t_9760_gosen_test(){
        assert_eq!(test_gosen_money_number(9760),-6);
    }
    #[test]
    fn t_9760_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9760),-4);
    }
    // 9770
    #[test]
    fn t_9770_ju_test(){
        assert_eq!(test_ju_money_number(9770),15);
    }
    #[test]
    fn t_9770_goju_test(){
        assert_eq!(test_goju_money_number(9770),-3);
    }
    #[test]
    fn t_9770_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9770),-7);
    }
    #[test]
    fn t_9770_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9770),-6);
    }
    #[test]
    fn t_9770_sen_test(){
        assert_eq!(test_sen_money_number(9770),-24);
    }
    #[test]
    fn t_9770_gosen_test(){
        assert_eq!(test_gosen_money_number(9770),-6);
    }
    #[test]
    fn t_9770_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9770),-4);
    }
    // 9780
    #[test]
    fn t_9780_ju_test(){
        assert_eq!(test_ju_money_number(9780),5);
    }
    #[test]
    fn t_9780_goju_test(){
        assert_eq!(test_goju_money_number(9780),-3);
    }
    #[test]
    fn t_9780_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9780),-7);
    }
    #[test]
    fn t_9780_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9780),-6);
    }
    #[test]
    fn t_9780_sen_test(){
        assert_eq!(test_sen_money_number(9780),-24);
    }
    #[test]
    fn t_9780_gosen_test(){
        assert_eq!(test_gosen_money_number(9780),-6);
    }
    #[test]
    fn t_9780_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9780),-4);
    }
    // 9790
    #[test]
    fn t_9790_ju_test(){
        assert_eq!(test_ju_money_number(9790),-5);
    }
    #[test]
    fn t_9790_goju_test(){
        assert_eq!(test_goju_money_number(9790),-3);
    }
    #[test]
    fn t_9790_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9790),-7);
    }
    #[test]
    fn t_9790_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9790),-6);
    }
    #[test]
    fn t_9790_sen_test(){
        assert_eq!(test_sen_money_number(9790),-24);
    }
    #[test]
    fn t_9790_gosen_test(){
        assert_eq!(test_gosen_money_number(9790),-6);
    }
    #[test]
    fn t_9790_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9790),-4);
    }
    // 9800
    #[test]
    fn t_9800_ju_test(){
        assert_eq!(test_ju_money_number(9800),0);
    }
    #[test]
    fn t_9800_goju_test(){
        assert_eq!(test_goju_money_number(9800),0);
    }
    #[test]
    fn t_9800_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9800),-5);
    }
    #[test]
    fn t_9800_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9800),-5);
    }
    #[test]
    fn t_9800_sen_test(){
        assert_eq!(test_sen_money_number(9800),-20);
    }
    #[test]
    fn t_9800_gosen_test(){
        assert_eq!(test_gosen_money_number(9800),-5);
    }
    #[test]
    fn t_9800_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9800),-5);
    }
    // 9810
    #[test]
    fn t_9810_ju_test(){
        assert_eq!(test_ju_money_number(9810),30);
    }
    #[test]
    fn t_9810_goju_test(){
        assert_eq!(test_goju_money_number(9810),4);
    }
    #[test]
    fn t_9810_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9810),-16);
    }
    #[test]
    fn t_9810_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9810),-6);
    }
    #[test]
    fn t_9810_sen_test(){
        assert_eq!(test_sen_money_number(9810),-24);
    }
    #[test]
    fn t_9810_gosen_test(){
        assert_eq!(test_gosen_money_number(9810),-6);
    }
    #[test]
    fn t_9810_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9810),-4);
    }
    // 9820
    #[test]
    fn t_9820_ju_test(){
        assert_eq!(test_ju_money_number(9820),20);
    }
    #[test]
    fn t_9820_goju_test(){
        assert_eq!(test_goju_money_number(9820),4);
    }
    #[test]
    fn t_9820_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9820),-16);
    }
    #[test]
    fn t_9820_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9820),-6);
    }
    #[test]
    fn t_9820_sen_test(){
        assert_eq!(test_sen_money_number(9820),-24);
    }
    #[test]
    fn t_9820_gosen_test(){
        assert_eq!(test_gosen_money_number(9820),-6);
    }
    #[test]
    fn t_9820_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9820),-4);
    }
    // 9830
    #[test]
    fn t_9830_ju_test(){
        assert_eq!(test_ju_money_number(9830),10);
    }
    #[test]
    fn t_9830_goju_test(){
        assert_eq!(test_goju_money_number(9830),4);
    }
    #[test]
    fn t_9830_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9830),-16);
    }
    #[test]
    fn t_9830_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9830),-6);
    }
    #[test]
    fn t_9830_sen_test(){
        assert_eq!(test_sen_money_number(9830),-24);
    }
    #[test]
    fn t_9830_gosen_test(){
        assert_eq!(test_gosen_money_number(9830),-6);
    }
    #[test]
    fn t_9830_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9830),-4);
    }
    // 9840
    #[test]
    fn t_9840_ju_test(){
        assert_eq!(test_ju_money_number(9840),0);
    }
    #[test]
    fn t_9840_goju_test(){
        assert_eq!(test_goju_money_number(9840),4);
    }
    #[test]
    fn t_9840_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9840),-16);
    }
    #[test]
    fn t_9840_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9840),-6);
    }
    #[test]
    fn t_9840_sen_test(){
        assert_eq!(test_sen_money_number(9840),-24);
    }
    #[test]
    fn t_9840_gosen_test(){
        assert_eq!(test_gosen_money_number(9840),-6);
    }
    #[test]
    fn t_9840_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9840),-4);
    }
    // 9850
    #[test]
    fn t_9850_ju_test(){
        assert_eq!(test_ju_money_number(9850),0);
    }
    #[test]
    fn t_9850_goju_test(){
        assert_eq!(test_goju_money_number(9850),4);
    }
    #[test]
    fn t_9850_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9850),-17);
    }
    #[test]
    fn t_9850_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9850),-6);
    }
    #[test]
    fn t_9850_sen_test(){
        assert_eq!(test_sen_money_number(9850),-24);
    }
    #[test]
    fn t_9850_gosen_test(){
        assert_eq!(test_gosen_money_number(9850),-6);
    }
    #[test]
    fn t_9850_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9850),-4);
    }
    // 9860
    #[test]
    fn t_9860_ju_test(){
        assert_eq!(test_ju_money_number(9860),25);
    }
    #[test]
    fn t_9860_goju_test(){
        assert_eq!(test_goju_money_number(9860),-3);
    }
    #[test]
    fn t_9860_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9860),-17);
    }
    #[test]
    fn t_9860_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9860),-6);
    }
    #[test]
    fn t_9860_sen_test(){
        assert_eq!(test_sen_money_number(9860),-24);
    }
    #[test]
    fn t_9860_gosen_test(){
        assert_eq!(test_gosen_money_number(9860),-6);
    }
    #[test]
    fn t_9860_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9860),-4);
    }
    // 9870
    #[test]
    fn t_9870_ju_test(){
        assert_eq!(test_ju_money_number(9870),15);
    }
    #[test]
    fn t_9870_goju_test(){
        assert_eq!(test_goju_money_number(9870),-3);
    }
    #[test]
    fn t_9870_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9870),-17);
    }
    #[test]
    fn t_9870_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9870),-6);
    }
    #[test]
    fn t_9870_sen_test(){
        assert_eq!(test_sen_money_number(9870),-24);
    }
    #[test]
    fn t_9870_gosen_test(){
        assert_eq!(test_gosen_money_number(9870),-6);
    }
    #[test]
    fn t_9870_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9870),-4);
    }
    // 9880
    #[test]
    fn t_9880_ju_test(){
        assert_eq!(test_ju_money_number(9880),5);
    }
    #[test]
    fn t_9880_goju_test(){
        assert_eq!(test_goju_money_number(9880),-3);
    }
    #[test]
    fn t_9880_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9880),-17);
    }
    #[test]
    fn t_9880_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9880),-6);
    }
    #[test]
    fn t_9880_sen_test(){
        assert_eq!(test_sen_money_number(9880),-24);
    }
    #[test]
    fn t_9880_gosen_test(){
        assert_eq!(test_gosen_money_number(9880),-6);
    }
    #[test]
    fn t_9880_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9880),-4);
    }
    // 9890
    #[test]
    fn t_9890_ju_test(){
        assert_eq!(test_ju_money_number(9890),-5);
    }
    #[test]
    fn t_9890_goju_test(){
        assert_eq!(test_goju_money_number(9890),-3);
    }
    #[test]
    fn t_9890_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9890),-17);
    }
    #[test]
    fn t_9890_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9890),-6);
    }
    #[test]
    fn t_9890_sen_test(){
        assert_eq!(test_sen_money_number(9890),-24);
    }
    #[test]
    fn t_9890_gosen_test(){
        assert_eq!(test_gosen_money_number(9890),-6);
    }
    #[test]
    fn t_9890_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9890),-4);
    }
    // 9900
    #[test]
    fn t_9900_ju_test(){
        assert_eq!(test_ju_money_number(9900),0);
    }
    #[test]
    fn t_9900_goju_test(){
        assert_eq!(test_goju_money_number(9900),0);
    }
    #[test]
    fn t_9900_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9900),-15);
    }
    #[test]
    fn t_9900_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9900),-5);
    }
    #[test]
    fn t_9900_sen_test(){
        assert_eq!(test_sen_money_number(9900),-20);
    }
    #[test]
    fn t_9900_gosen_test(){
        assert_eq!(test_gosen_money_number(9800),-5);
    }
    #[test]
    fn t_9900_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9800),-5);
    }
    // 9910
    #[test]
    fn t_9910_ju_test(){
        assert_eq!(test_ju_money_number(9910),25);
    }
    #[test]
    fn t_9910_goju_test(){
        assert_eq!(test_goju_money_number(9910),1);
    }
    #[test]
    fn t_9910_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9910),-24);
    }
    #[test]
    fn t_9910_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9910),-6);
    }
    #[test]
    fn t_9910_sen_test(){
        assert_eq!(test_sen_money_number(9910),-24);
    }
    #[test]
    fn t_9910_gosen_test(){
        assert_eq!(test_gosen_money_number(9910),-6);
    }
    #[test]
    fn t_9910_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9910),-4);
    }
    // 9920
    #[test]
    fn t_9920_ju_test(){
        assert_eq!(test_ju_money_number(9920),15);
    }
    #[test]
    fn t_9920_goju_test(){
        assert_eq!(test_goju_money_number(9920),1);
    }
    #[test]
    fn t_9920_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9920),-24);
    }
    #[test]
    fn t_9920_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9920),-6);
    }
    #[test]
    fn t_9920_sen_test(){
        assert_eq!(test_sen_money_number(9920),-24);
    }
    #[test]
    fn t_9920_gosen_test(){
        assert_eq!(test_gosen_money_number(9920),-6);
    }
    #[test]
    fn t_9920_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9920),-4);
    }
    // 9930
    #[test]
    fn t_9930_ju_test(){
        assert_eq!(test_ju_money_number(9930),5);
    }
    #[test]
    fn t_9930_goju_test(){
        assert_eq!(test_goju_money_number(9930),1);
    }
    #[test]
    fn t_9930_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9930),-24);
    }
    #[test]
    fn t_9930_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9930),-6);
    }
    #[test]
    fn t_9930_sen_test(){
        assert_eq!(test_sen_money_number(9930),-24);
    }
    #[test]
    fn t_9930_gosen_test(){
        assert_eq!(test_gosen_money_number(9930),-6);
    }
    #[test]
    fn t_9930_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9930),-4);
    }
    // 9940
    #[test]
    fn t_9940_ju_test(){
        assert_eq!(test_ju_money_number(9940),-5);
    }
    #[test]
    fn t_9940_goju_test(){
        assert_eq!(test_goju_money_number(9940),1);
    }
    #[test]
    fn t_9940_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9940),-24);
    }
    #[test]
    fn t_9940_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9940),-6);
    }
    #[test]
    fn t_9940_sen_test(){
        assert_eq!(test_sen_money_number(9940),-24);
    }
    #[test]
    fn t_9940_gosen_test(){
        assert_eq!(test_gosen_money_number(9940),-6);
    }
    #[test]
    fn t_9940_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9940),-4);
    }
    // 9950
    #[test]
    fn t_9950_ju_test(){
        assert_eq!(test_ju_money_number(9950),0);
    }
    #[test]
    fn t_9950_goju_test(){
        assert_eq!(test_goju_money_number(9950),0);
    }
    #[test]
    fn t_9950_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9950),-20);
    }
    #[test]
    fn t_9950_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9950),-5);
    }
    #[test]
    fn t_9950_sen_test(){
        assert_eq!(test_sen_money_number(9950),-20);
    }
    #[test]
    fn t_9950_gosen_test(){
        assert_eq!(test_gosen_money_number(9950),-5);
    }
    #[test]
    fn t_9950_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9950),-5);
    }
    // 9960
    #[test]
    fn t_9960_ju_test(){
        assert_eq!(test_ju_money_number(9960),15);
    }
    #[test]
    fn t_9960_goju_test(){
        assert_eq!(test_goju_money_number(9960),-5);
    }
    #[test]
    fn t_9960_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9960),-20);
    }
    #[test]
    fn t_9960_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9960),-5);
    }
    #[test]
    fn t_9960_sen_test(){
        assert_eq!(test_sen_money_number(9960),-20);
    }
    #[test]
    fn t_9960_gosen_test(){
        assert_eq!(test_gosen_money_number(9960),-5);
    }
    #[test]
    fn t_9960_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9960),-5);
    }
    // 9970
    #[test]
    fn t_9970_ju_test(){
        assert_eq!(test_ju_money_number(9970),5);
    }
    #[test]
    fn t_9970_goju_test(){
        assert_eq!(test_goju_money_number(9970),-5);
    }
    #[test]
    fn t_9970_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9970),-20);
    }
    #[test]
    fn t_9970_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9970),-5);
    }
    #[test]
    fn t_9970_sen_test(){
        assert_eq!(test_sen_money_number(9970),-20);
    }
    #[test]
    fn t_9970_gosen_test(){
        assert_eq!(test_gosen_money_number(9970),-5);
    }
    #[test]
    fn t_9970_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9970),-5);
    }
    // 9980
    #[test]
    fn t_9980_ju_test(){
        assert_eq!(test_ju_money_number(9980),-5);
    }
    #[test]
    fn t_9980_goju_test(){
        assert_eq!(test_goju_money_number(9980),-5);
    }
    #[test]
    fn t_9980_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9980),-20);
    }
    #[test]
    fn t_9980_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9980),-5);
    }
    #[test]
    fn t_9980_sen_test(){
        assert_eq!(test_sen_money_number(9980),-20);
    }
    #[test]
    fn t_9980_gosen_test(){
        assert_eq!(test_gosen_money_number(9980),-5);
    }
    #[test]
    fn t_9980_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9980),-5);
    }
    // 9990
    #[test]
    fn t_9990_ju_test(){
        assert_eq!(test_ju_money_number(9990),-15);
    }
    #[test]
    fn t_9990_goju_test(){
        assert_eq!(test_goju_money_number(9990),-5);
    }
    #[test]
    fn t_9990_hyaku_test(){
        assert_eq!(test_hyaku_money_number(9990),-20);
    }
    #[test]
    fn t_9990_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(9990),-5);
    }
    #[test]
    fn t_9990_sen_test(){
        assert_eq!(test_sen_money_number(9990),-20);
    }
    #[test]
    fn t_9990_gosen_test(){
        assert_eq!(test_gosen_money_number(9990),-5);
    }
    #[test]
    fn t_9990_ichiman_test(){
        assert_eq!(test_ichiman_money_number(9990),-5);
    }
    #[test]
    fn t_10000_hyaku_test(){
        assert_eq!(test_hyaku_money_number(10000),0);
    }
    #[test]
    fn t_10000_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(10000),0);
    }
    #[test]
    fn t_10000_sen_test(){
        assert_eq!(test_sen_money_number(10000),0);
    }
    #[test]
    fn t_10000_gosen_test(){
        assert_eq!(test_gosen_money_number(10000),0);
    }
    #[test]
    fn t_10000_ichiman_test(){
        assert_eq!(test_ichiman_money_number(10000),-10);
    }
    #[test]
    fn t_10100_hyaku_test(){
        assert_eq!(test_hyaku_money_number(10100),30);
    }
    #[test]
    fn t_10100_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(10100),4);
    }
    #[test]
    fn t_10100_sen_test(){
        assert_eq!(test_sen_money_number(10100),14);
    }
    #[test]
    fn t_10100_gosen_test(){
        assert_eq!(test_gosen_money_number(10100),-4);
    }
    #[test]
    fn t_10200_hyaku_test(){
        assert_eq!(test_hyaku_money_number(10200),20);
    }
    #[test]
    fn t_10200_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(10200),4);
    }
    #[test]
    fn t_10200_sen_test(){
        assert_eq!(test_sen_money_number(10200),14);
    }
    #[test]
    fn t_10200_gosen_test(){
        assert_eq!(test_gosen_money_number(10200),-4);
    }
    #[test]
    fn t_10300_hyaku_test(){
        assert_eq!(test_hyaku_money_number(10300),10);
    }
    #[test]
    fn t_10300_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(10300),4);
    }
    #[test]
    fn t_10300_sen_test(){
        assert_eq!(test_sen_money_number(10300),14);
    }
    #[test]
    fn t_10300_gosen_test(){
        assert_eq!(test_gosen_money_number(10300),-4);
    }
    #[test]
    fn t_10400_hyaku_test(){
        assert_eq!(test_hyaku_money_number(10400),0);
    }
    #[test]
    fn t_10400_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(10400),4);
    }
    #[test]
    fn t_10400_sen_test(){
        assert_eq!(test_sen_money_number(10400),14);
    }
    #[test]
    fn t_10400_gosen_test(){
        assert_eq!(test_gosen_money_number(10400),-4);
    }
    #[test]
    fn t_10500_hyaku_test(){
        assert_eq!(test_hyaku_money_number(10500),0);
    }
    #[test]
    fn t_10500_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(10500),4);
    }
    #[test]
    fn t_10500_sen_test(){
        assert_eq!(test_sen_money_number(10500),13);
    }
    #[test]
    fn t_10500_gosen_test(){
        assert_eq!(test_gosen_money_number(10500),-4);
    }
    #[test]
    fn t_10600_hyaku_test(){
        assert_eq!(test_hyaku_money_number(10600),25);
    }
    #[test]
    fn t_10600_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(10600),-3);
    }
    #[test]
    fn t_10600_sen_test(){
        assert_eq!(test_sen_money_number(10600),13);
    }
    #[test]
    fn t_10600_gosen_test(){
        assert_eq!(test_gosen_money_number(10600),-4);
    }
    #[test]
    fn t_10700_hyaku_test(){
        assert_eq!(test_hyaku_money_number(10700),15);
    }
    #[test]
    fn t_10700_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(10700),-3);
    }
    #[test]
    fn t_10700_sen_test(){
        assert_eq!(test_sen_money_number(10700),13);
    }
    #[test]
    fn t_10700_gosen_test(){
        assert_eq!(test_gosen_money_number(10700),-4);
    }
    #[test]
    fn t_10800_hyaku_test(){
        assert_eq!(test_hyaku_money_number(10800),5);
    }
    #[test]
    fn t_10800_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(10800),-3);
    }
    #[test]
    fn t_10800_sen_test(){
        assert_eq!(test_sen_money_number(10800),13);
    }
    #[test]
    fn t_10800_gosen_test(){
        assert_eq!(test_gosen_money_number(10800),-4);
    }
    #[test]
    fn t_10900_hyaku_test(){
        assert_eq!(test_hyaku_money_number(10900),-5);
    }
    #[test]
    fn t_10900_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(10900),-3);
    }
    #[test]
    fn t_10900_sen_test(){
        assert_eq!(test_sen_money_number(10900),13);
    }
    #[test]
    fn t_10900_gosen_test(){
        assert_eq!(test_gosen_money_number(10900),-4);
    }
    #[test]
    fn t_11000_hyaku_test(){
        assert_eq!(test_hyaku_money_number(11000),0);
    }
    #[test]
    fn t_11000_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(11000),0);
    }
    #[test]
    fn t_11000_sen_test(){
        assert_eq!(test_sen_money_number(11000),15);
    }
    #[test]
    fn t_11000_gosen_test(){
        assert_eq!(test_gosen_money_number(11000),-5);
    }
    #[test]
    fn t_11100_hyaku_test(){
        assert_eq!(test_hyaku_money_number(11100),30);
    }
    #[test]
    fn t_11100_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(11100),4);
    }
    #[test]
    fn t_11100_sen_test(){
        assert_eq!(test_sen_money_number(11100),4);
    }
    #[test]
    fn t_11100_gosen_test(){
        assert_eq!(test_gosen_money_number(11100),-4);
    }
    #[test]
    fn t_11200_hyaku_test(){
        assert_eq!(test_hyaku_money_number(11200),20);
    }
    #[test]
    fn t_11200_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(11200),4);
    }
    #[test]
    fn t_11200_sen_test(){
        assert_eq!(test_sen_money_number(11200),4);
    }
    #[test]
    fn t_11200_gosen_test(){
        assert_eq!(test_gosen_money_number(11200),-4);
    }
    #[test]
    fn t_11300_hyaku_test(){
        assert_eq!(test_hyaku_money_number(11300),10);
    }
    #[test]
    fn t_11300_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(11300),4);
    }
    #[test]
    fn t_11300_sen_test(){
        assert_eq!(test_sen_money_number(11300),4);
    }
    #[test]
    fn t_11300_gosen_test(){
        assert_eq!(test_gosen_money_number(11300),-4);
    }
    #[test]
    fn t_11400_hyaku_test(){
        assert_eq!(test_hyaku_money_number(11400),0);
    }
    #[test]
    fn t_11400_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(11400),4);
    }
    #[test]
    fn t_11400_sen_test(){
        assert_eq!(test_sen_money_number(11400),4);
    }
    #[test]
    fn t_11400_gosen_test(){
        assert_eq!(test_gosen_money_number(11400),-4);
    }
    #[test]
    fn t_11500_hyaku_test(){
        assert_eq!(test_hyaku_money_number(11500),0);
    }
    #[test]
    fn t_11500_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(11500),4);
    }
    #[test]
    fn t_11500_sen_test(){
        assert_eq!(test_sen_money_number(11500),3);
    }
    #[test]
    fn t_11500_gosen_test(){
        assert_eq!(test_gosen_money_number(11500),-4);
    }
    #[test]
    fn t_11600_hyaku_test(){
        assert_eq!(test_hyaku_money_number(11600),25);
    }
    #[test]
    fn t_11600_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(11600),-3);
    }
    #[test]
    fn t_11600_sen_test(){
        assert_eq!(test_sen_money_number(11600),3);
    }
    #[test]
    fn t_11600_gosen_test(){
        assert_eq!(test_gosen_money_number(11600),-4);
    }
    #[test]
    fn t_11700_hyaku_test(){
        assert_eq!(test_hyaku_money_number(11700),15);
    }
    #[test]
    fn t_11700_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(11700),-3);
    }
    #[test]
    fn t_11700_sen_test(){
        assert_eq!(test_sen_money_number(11700),3);
    }
    #[test]
    fn t_11700_gosen_test(){
        assert_eq!(test_gosen_money_number(11700),-4);
    }
    #[test]
    fn t_11800_hyaku_test(){
        assert_eq!(test_hyaku_money_number(11800),5);
    }
    #[test]
    fn t_11800_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(11800),-3);
    }
    #[test]
    fn t_11800_sen_test(){
        assert_eq!(test_sen_money_number(11800),3);
    }
    #[test]
    fn t_11800_gosen_test(){
        assert_eq!(test_gosen_money_number(11800),-4);
    }
    #[test]
    fn t_11900_hyaku_test(){
        assert_eq!(test_hyaku_money_number(11900),-5);
    }
    #[test]
    fn t_11900_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(11900),-3);
    }
    #[test]
    fn t_11900_sen_test(){
        assert_eq!(test_sen_money_number(11900),3);
    }
    #[test]
    fn t_11900_gosen_test(){
        assert_eq!(test_gosen_money_number(11900),-4);
    }
    #[test]
    fn t_12000_hyaku_test(){
        assert_eq!(test_hyaku_money_number(12000),0);
    }
    #[test]
    fn t_12000_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(12000),0);
    }
    #[test]
    fn t_12000_sen_test(){
        assert_eq!(test_sen_money_number(12000),5);
    }
    #[test]
    fn t_12000_gosen_test(){
        assert_eq!(test_gosen_money_number(12000),-5);
    }
    #[test]
    fn t_12100_hyaku_test(){
        assert_eq!(test_hyaku_money_number(12100),30);
    }
    #[test]
    fn t_12100_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(12100),4);
    }
    #[test]
    fn t_12100_sen_test(){
        assert_eq!(test_sen_money_number(12100),-6);
    }
    #[test]
    fn t_12100_gosen_test(){
        assert_eq!(test_gosen_money_number(12100),-4);
    }
    #[test]
    fn t_12200_hyaku_test(){
        assert_eq!(test_hyaku_money_number(12200),20);
    }
    #[test]
    fn t_12200_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(12200),4);
    }
    #[test]
    fn t_12200_sen_test(){
        assert_eq!(test_sen_money_number(12200),-6);
    }
    #[test]
    fn t_12200_gosen_test(){
        assert_eq!(test_gosen_money_number(12200),-4);
    }
    #[test]
    fn t_12300_hyaku_test(){
        assert_eq!(test_hyaku_money_number(12300),10);
    }
    #[test]
    fn t_12300_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(12300),4);
    }
    #[test]
    fn t_12300_sen_test(){
        assert_eq!(test_sen_money_number(12300),-6);
    }
    #[test]
    fn t_12300_gosen_test(){
        assert_eq!(test_gosen_money_number(12300),-4);
    }
    #[test]
    fn t_12400_hyaku_test(){
        assert_eq!(test_hyaku_money_number(12400),0);
    }
    #[test]
    fn t_12400_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(12400),4);
    }
    #[test]
    fn t_12400_sen_test(){
        assert_eq!(test_sen_money_number(12400),-6);
    }
    #[test]
    fn t_12400_gosen_test(){
        assert_eq!(test_gosen_money_number(12400),-4);
    }
    #[test]
    fn t_12500_hyaku_test(){
        assert_eq!(test_hyaku_money_number(12500),0);
    }
    #[test]
    fn t_12500_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(12500),4);
    }
    #[test]
    fn t_12500_sen_test(){
        assert_eq!(test_sen_money_number(12500),-7);
    }
    #[test]
    fn t_12500_gosen_test(){
        assert_eq!(test_gosen_money_number(12500),-4);
    }
    #[test]
    fn t_12600_hyaku_test(){
        assert_eq!(test_hyaku_money_number(12600),25);
    }
    #[test]
    fn t_12600_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(12600),-3);
    }
    #[test]
    fn t_12600_sen_test(){
        assert_eq!(test_sen_money_number(12600),-7);
    }
    #[test]
    fn t_12600_gosen_test(){
        assert_eq!(test_gosen_money_number(12600),-4);
    }
    #[test]
    fn t_12700_hyaku_test(){
        assert_eq!(test_hyaku_money_number(12700),15);
    }
    #[test]
    fn t_12700_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(12700),-3);
    }
    #[test]
    fn t_12700_sen_test(){
        assert_eq!(test_sen_money_number(12700),-7);
    }
    #[test]
    fn t_12700_gosen_test(){
        assert_eq!(test_gosen_money_number(12700),-4);
    }
    #[test]
    fn t_12800_hyaku_test(){
        assert_eq!(test_hyaku_money_number(12800),5);
    }
    #[test]
    fn t_12800_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(12800),-3);
    }
    #[test]
    fn t_12800_sen_test(){
        assert_eq!(test_sen_money_number(12800),-7);
    }
    #[test]
    fn t_12800_gosen_test(){
        assert_eq!(test_gosen_money_number(12800),-4);
    }
    #[test]
    fn t_12900_hyaku_test(){
        assert_eq!(test_hyaku_money_number(12900),-5);
    }
    #[test]
    fn t_12900_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(12900),-3);
    }
    #[test]
    fn t_12900_sen_test(){
        assert_eq!(test_sen_money_number(12900),-7);
    }
    #[test]
    fn t_12900_gosen_test(){
        assert_eq!(test_gosen_money_number(12900),-4);
    }
    #[test]
    fn t_13000_hyaku_test(){
        assert_eq!(test_hyaku_money_number(13000),0);
    }
    #[test]
    fn t_13000_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(13000),0);
    }
    #[test]
    fn t_13000_sen_test(){
        assert_eq!(test_sen_money_number(13000),-5);
    }
    #[test]
    fn t_13000_gosen_test(){
        assert_eq!(test_gosen_money_number(13000),-5);
    }
    #[test]
    fn t_13100_hyaku_test(){
        assert_eq!(test_hyaku_money_number(13100),30);
    }
    #[test]
    fn t_13100_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(13100),4);
    }
    #[test]
    fn t_13100_sen_test(){
        assert_eq!(test_sen_money_number(13100),-16);
    }
    #[test]
    fn t_13100_gosen_test(){
        assert_eq!(test_gosen_money_number(13100),-4);
    }
    #[test]
    fn t_13200_hyaku_test(){
        assert_eq!(test_hyaku_money_number(13200),20);
    }
    #[test]
    fn t_13200_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(13200),4);
    }
    #[test]
    fn t_13200_sen_test(){
        assert_eq!(test_sen_money_number(13200),-16);
    }
    #[test]
    fn t_13200_gosen_test(){
        assert_eq!(test_gosen_money_number(13200),-4);
    }
    #[test]
    fn t_13300_hyaku_test(){
        assert_eq!(test_hyaku_money_number(13300),10);
    }
    #[test]
    fn t_13300_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(13300),4);
    }
    #[test]
    fn t_13300_sen_test(){
        assert_eq!(test_sen_money_number(13300),-16);
    }
    #[test]
    fn t_13300_gosen_test(){
        assert_eq!(test_gosen_money_number(13300),-4);
    }
    #[test]
    fn t_13400_hyaku_test(){
        assert_eq!(test_hyaku_money_number(13400),0);
    }
    #[test]
    fn t_13400_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(13400),4);
    }
    #[test]
    fn t_13400_sen_test(){
        assert_eq!(test_sen_money_number(13400),-16);
    }
    #[test]
    fn t_13400_gosen_test(){
        assert_eq!(test_gosen_money_number(13400),-4);
    }
    #[test]
    fn t_13500_hyaku_test(){
        assert_eq!(test_hyaku_money_number(13500),0);
    }
    #[test]
    fn t_13500_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(13500),4);
    }
    #[test]
    fn t_13500_sen_test(){
        assert_eq!(test_sen_money_number(13500),-17);
    }
    #[test]
    fn t_13500_gosen_test(){
        assert_eq!(test_gosen_money_number(13500),-4);
    }
    #[test]
    fn t_13600_hyaku_test(){
        assert_eq!(test_hyaku_money_number(13600),25);
    }
    #[test]
    fn t_13600_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(13600),-3);
    }
    #[test]
    fn t_13600_sen_test(){
        assert_eq!(test_sen_money_number(13600),-17);
    }
    #[test]
    fn t_13600_gosen_test(){
        assert_eq!(test_gosen_money_number(13600),-4);
    }
    #[test]
    fn t_13700_hyaku_test(){
        assert_eq!(test_hyaku_money_number(13700),15);
    }
    #[test]
    fn t_13700_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(13700),-3);
    }
    #[test]
    fn t_13700_sen_test(){
        assert_eq!(test_sen_money_number(13700),-17);
    }
    #[test]
    fn t_13700_gosen_test(){
        assert_eq!(test_gosen_money_number(13700),-4);
    }
    #[test]
    fn t_13800_hyaku_test(){
        assert_eq!(test_hyaku_money_number(13800),5);
    }
    #[test]
    fn t_13800_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(13800),-3);
    }
    #[test]
    fn t_13800_sen_test(){
        assert_eq!(test_sen_money_number(13800),-17);
    }
    #[test]
    fn t_13800_gosen_test(){
        assert_eq!(test_gosen_money_number(13800),-4);
    }
    #[test]
    fn t_13900_hyaku_test(){
        assert_eq!(test_hyaku_money_number(13900),-5);
    }
    #[test]
    fn t_13900_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(13900),-3);
    }
    #[test]
    fn t_13900_sen_test(){
        assert_eq!(test_sen_money_number(13900),-17);
    }
    #[test]
    fn t_13900_gosen_test(){
        assert_eq!(test_gosen_money_number(13900),-4);
    }
    #[test]
    fn t_14000_hyaku_test(){
        assert_eq!(test_hyaku_money_number(14000),0);
    }
    #[test]
    fn t_14000_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(14000),0);
    }
    #[test]
    fn t_14000_sen_test(){
        assert_eq!(test_sen_money_number(14000),-15);
    }
    #[test]
    fn t_14000_gosen_test(){
        assert_eq!(test_gosen_money_number(14000),-5);
    }
    #[test]
    fn t_14100_hyaku_test(){
        assert_eq!(test_hyaku_money_number(14100),25);
    }
    #[test]
    fn t_14100_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(14100),1);
    }
    #[test]
    fn t_14100_sen_test(){
        assert_eq!(test_sen_money_number(14100),-24);
    }
    #[test]
    fn t_14100_gosen_test(){
        assert_eq!(test_gosen_money_number(14100),-4);
    }
    #[test]
    fn t_14200_hyaku_test(){
        assert_eq!(test_hyaku_money_number(14200),15);
    }
    #[test]
    fn t_14200_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(14200),1);
    }
    #[test]
    fn t_14200_sen_test(){
        assert_eq!(test_sen_money_number(14200),-24);
    }
    #[test]
    fn t_14200_gosen_test(){
        assert_eq!(test_gosen_money_number(14200),-4);
    }
    #[test]
    fn t_14300_hyaku_test(){
        assert_eq!(test_hyaku_money_number(14300),5);
    }
    #[test]
    fn t_14300_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(14300),1);
    }
    #[test]
    fn t_14300_sen_test(){
        assert_eq!(test_sen_money_number(14300),-24);
    }
    #[test]
    fn t_14300_gosen_test(){
        assert_eq!(test_gosen_money_number(14300),-4);
    }
    #[test]
    fn t_14400_hyaku_test(){
        assert_eq!(test_hyaku_money_number(14400),-5);
    }
    #[test]
    fn t_14400_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(14400),1);
    }
    #[test]
    fn t_14400_sen_test(){
        assert_eq!(test_sen_money_number(14400),-24);
    }
    #[test]
    fn t_14400_gosen_test(){
        assert_eq!(test_gosen_money_number(14400),-4);
    }
    #[test]
    fn t_14500_hyaku_test(){
        assert_eq!(test_hyaku_money_number(14500),0);
    }
    #[test]
    fn t_14500_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(14500),0);
    }
    #[test]
    fn t_14500_sen_test(){
        assert_eq!(test_sen_money_number(14500),-20);
    }
    #[test]
    fn t_14500_gosen_test(){
        assert_eq!(test_gosen_money_number(14500),-5);
    }
    #[test]
    fn t_14600_hyaku_test(){
        assert_eq!(test_hyaku_money_number(14600),15);
    }
    #[test]
    fn t_14600_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(14600),-5);
    }
    #[test]
    fn t_14600_sen_test(){
        assert_eq!(test_sen_money_number(14600),-20);
    }
    #[test]
    fn t_14600_gosen_test(){
        assert_eq!(test_gosen_money_number(14600),-5);
    }
    #[test]
    fn t_14700_hyaku_test(){
        assert_eq!(test_hyaku_money_number(14700),5);
    }
    #[test]
    fn t_14700_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(14700),-5);
    }
    #[test]
    fn t_14700_sen_test(){
        assert_eq!(test_sen_money_number(14700),-20);
    }
    #[test]
    fn t_14700_gosen_test(){
        assert_eq!(test_gosen_money_number(14700),-5);
    }
    #[test]
    fn t_14800_hyaku_test(){
        assert_eq!(test_hyaku_money_number(14800),-5);
    }
    #[test]
    fn t_14800_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(14800),-5);
    }
    #[test]
    fn t_14800_sen_test(){
        assert_eq!(test_sen_money_number(14800),-20);
    }
    #[test]
    fn t_14800_gosen_test(){
        assert_eq!(test_gosen_money_number(14800),-5);
    }
    #[test]
    fn t_14900_hyaku_test(){
        assert_eq!(test_hyaku_money_number(14900),-15);
    }
    #[test]
    fn t_14900_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(14900),-5);
    }
    #[test]
    fn t_14900_sen_test(){
        assert_eq!(test_sen_money_number(14900),-20);
    }
    #[test]
    fn t_14900_gosen_test(){
        assert_eq!(test_gosen_money_number(14900),-5);
    }
    #[test]
    fn t_15000_hyaku_test(){
        assert_eq!(test_hyaku_money_number(15000),0);
    }
    #[test]
    fn t_15000_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(15000),0);
    }
    #[test]
    fn t_15000_sen_test(){
        assert_eq!(test_sen_money_number(15000),0);
    }
    #[test]
    fn t_15000_gosen_test(){
        assert_eq!(test_gosen_money_number(15000),0);
    }
    #[test]
    fn t_15100_hyaku_test(){
        assert_eq!(test_hyaku_money_number(15100),30);
    }
    #[test]
    fn t_15100_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(15100),4);
    }
    #[test]
    fn t_15100_sen_test(){
        assert_eq!(test_sen_money_number(15100),14);
    }
    #[test]
    fn t_15100_gosen_test(){
        assert_eq!(test_gosen_money_number(15100),-6);
    }
    #[test]
    fn t_15200_hyaku_test(){
        assert_eq!(test_hyaku_money_number(15200),20);
    }
    #[test]
    fn t_15200_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(15200),4);
    }
    #[test]
    fn t_15200_sen_test(){
        assert_eq!(test_sen_money_number(15200),14);
    }
    #[test]
    fn t_15200_gosen_test(){
        assert_eq!(test_gosen_money_number(15200),-6);
    }
    #[test]
    fn t_15300_hyaku_test(){
        assert_eq!(test_hyaku_money_number(15300),10);
    }
    #[test]
    fn t_15300_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(15300),4);
    }
    #[test]
    fn t_15300_sen_test(){
        assert_eq!(test_sen_money_number(15300),14);
    }
    #[test]
    fn t_15300_gosen_test(){
        assert_eq!(test_gosen_money_number(15300),-6);
    }
    #[test]
    fn t_15400_hyaku_test(){
        assert_eq!(test_hyaku_money_number(15400),0);
    }
    #[test]
    fn t_15400_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(15400),4);
    }
    #[test]
    fn t_15400_sen_test(){
        assert_eq!(test_sen_money_number(15400),14);
    }
    #[test]
    fn t_15400_gosen_test(){
        assert_eq!(test_gosen_money_number(15400),-6);
    }
    #[test]
    fn t_15500_hyaku_test(){
        assert_eq!(test_hyaku_money_number(15500),0);
    }
    #[test]
    fn t_15500_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(15500),4);
    }
    #[test]
    fn t_15500_sen_test(){
        assert_eq!(test_sen_money_number(15500),13);
    }
    #[test]
    fn t_15500_gosen_test(){
        assert_eq!(test_gosen_money_number(15500),-6);
    }
    #[test]
    fn t_15600_hyaku_test(){
        assert_eq!(test_hyaku_money_number(15600),25);
    }
    #[test]
    fn t_15600_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(15600),-3);
    }
    #[test]
    fn t_15600_sen_test(){
        assert_eq!(test_sen_money_number(15600),13);
    }
    #[test]
    fn t_15600_gosen_test(){
        assert_eq!(test_gosen_money_number(15600),-6);
    }
    #[test]
    fn t_15700_hyaku_test(){
        assert_eq!(test_hyaku_money_number(15700),15);
    }
    #[test]
    fn t_15700_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(15700),-3);
    }
    #[test]
    fn t_15700_sen_test(){
        assert_eq!(test_sen_money_number(15700),13);
    }
    #[test]
    fn t_15700_gosen_test(){
        assert_eq!(test_gosen_money_number(15700),-6);
    }
    #[test]
    fn t_15800_hyaku_test(){
        assert_eq!(test_hyaku_money_number(15800),5);
    }
    #[test]
    fn t_15800_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(15800),-3);
    }
    #[test]
    fn t_15800_sen_test(){
        assert_eq!(test_sen_money_number(15800),13);
    }
    #[test]
    fn t_15800_gosen_test(){
        assert_eq!(test_gosen_money_number(15800),-6);
    }
    #[test]
    fn t_15900_hyaku_test(){
        assert_eq!(test_hyaku_money_number(15900),-5);
    }
    #[test]
    fn t_15900_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(15800),-3);
    }
    #[test]
    fn t_15900_sen_test(){
        assert_eq!(test_sen_money_number(15800),13);
    }
    #[test]
    fn t_15900_gosen_test(){
        assert_eq!(test_gosen_money_number(15800),-6);
    }
    #[test]
    fn t_16000_hyaku_test(){
        assert_eq!(test_hyaku_money_number(16000),0);
    }
    #[test]
    fn t_16000_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(16000),0);
    }
    #[test]
    fn t_16000_sen_test(){
        assert_eq!(test_sen_money_number(16000),15);
    }
    #[test]
    fn t_16000_gosen_test(){
        assert_eq!(test_gosen_money_number(16000),-5);
    }
    #[test]
    fn t_16100_hyaku_test(){
        assert_eq!(test_hyaku_money_number(16100),30);
    }
    #[test]
    fn t_16100_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(16100),4);
    }
    #[test]
    fn t_16100_sen_test(){
        assert_eq!(test_sen_money_number(16100),4);
    }
    #[test]
    fn t_16100_gosen_test(){
        assert_eq!(test_gosen_money_number(16100),-6);
    }
    #[test]
    fn t_16200_hyaku_test(){
        assert_eq!(test_hyaku_money_number(16200),20);
    }
    #[test]
    fn t_16200_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(16200),4);
    }
    #[test]
    fn t_16200_sen_test(){
        assert_eq!(test_sen_money_number(16200),4);
    }
    #[test]
    fn t_16200_gosen_test(){
        assert_eq!(test_gosen_money_number(16200),-6);
    }
    #[test]
    fn t_16300_hyaku_test(){
        assert_eq!(test_hyaku_money_number(16300),10);
    }
    #[test]
    fn t_16300_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(16300),4);
    }
    #[test]
    fn t_16300_sen_test(){
        assert_eq!(test_sen_money_number(16300),4);
    }
    #[test]
    fn t_16300_gosen_test(){
        assert_eq!(test_gosen_money_number(16300),-6);
    }
    #[test]
    fn t_16400_hyaku_test(){
        assert_eq!(test_hyaku_money_number(16400),0);
    }
    #[test]
    fn t_16400_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(16400),4);
    }
    #[test]
    fn t_16400_sen_test(){
        assert_eq!(test_sen_money_number(16400),4);
    }
    #[test]
    fn t_16400_gosen_test(){
        assert_eq!(test_gosen_money_number(16400),-6);
    }
    #[test]
    fn t_16500_hyaku_test(){
        assert_eq!(test_hyaku_money_number(16500),0);
    }
    #[test]
    fn t_16500_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(16500),4);
    }
    #[test]
    fn t_16500_sen_test(){
        assert_eq!(test_sen_money_number(16500),3);
    }
    #[test]
    fn t_16500_gosen_test(){
        assert_eq!(test_gosen_money_number(16500),-6);
    }
    #[test]
    fn t_16600_hyaku_test(){
        assert_eq!(test_hyaku_money_number(16600),25);
    }
    #[test]
    fn t_16600_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(16600),-3);
    }
    #[test]
    fn t_16600_sen_test(){
        assert_eq!(test_sen_money_number(16600),3);
    }
    #[test]
    fn t_16600_gosen_test(){
        assert_eq!(test_gosen_money_number(16600),-6);
    }
    #[test]
    fn t_16700_hyaku_test(){
        assert_eq!(test_hyaku_money_number(16700),15);
    }
    #[test]
    fn t_16700_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(16700),-3);
    }
    #[test]
    fn t_16700_sen_test(){
        assert_eq!(test_sen_money_number(16700),3);
    }
    #[test]
    fn t_16700_gosen_test(){
        assert_eq!(test_gosen_money_number(16700),-6);
    }
    #[test]
    fn t_16800_hyaku_test(){
        assert_eq!(test_hyaku_money_number(16800),5);
    }
    #[test]
    fn t_16800_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(16800),-3);
    }
    #[test]
    fn t_16800_sen_test(){
        assert_eq!(test_sen_money_number(16800),3);
    }
    #[test]
    fn t_16800_gosen_test(){
        assert_eq!(test_gosen_money_number(16800),-6);
    }
    #[test]
    fn t_16900_hyaku_test(){
        assert_eq!(test_hyaku_money_number(16900),-5);
    }
    #[test]
    fn t_16900_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(16900),-3);
    }
    #[test]
    fn t_16900_sen_test(){
        assert_eq!(test_sen_money_number(16900),3);
    }
    #[test]
    fn t_16900_gosen_test(){
        assert_eq!(test_gosen_money_number(16900),-6);
    }
    #[test]
    fn t_17000_hyaku_test(){
        assert_eq!(test_hyaku_money_number(17000),0);
    }
    #[test]
    fn t_17000_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(17000),0);
    }
    #[test]
    fn t_17000_sen_test(){
        assert_eq!(test_sen_money_number(17000),5);
    }
    #[test]
    fn t_17000_gosen_test(){
        assert_eq!(test_gosen_money_number(17000),-5);
    }
    #[test]
    fn t_17100_hyaku_test(){
        assert_eq!(test_hyaku_money_number(17100),30);
    }
    #[test]
    fn t_17100_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(17100),4);
    }
    #[test]
    fn t_17100_sen_test(){
        assert_eq!(test_sen_money_number(17100),-6);
    }
    #[test]
    fn t_17100_gosen_test(){
        assert_eq!(test_gosen_money_number(17100),-6);
    }
    #[test]
    fn t_17200_hyaku_test(){
        assert_eq!(test_hyaku_money_number(17200),20);
    }
    #[test]
    fn t_17200_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(17200),4);
    }
    #[test]
    fn t_17200_sen_test(){
        assert_eq!(test_sen_money_number(17200),-6);
    }
    #[test]
    fn t_17200_gosen_test(){
        assert_eq!(test_gosen_money_number(17200),-6);
    }
    #[test]
    fn t_17300_hyaku_test(){
        assert_eq!(test_hyaku_money_number(17300),10);
    }
    #[test]
    fn t_17300_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(17300),4);
    }
    #[test]
    fn t_17300_sen_test(){
        assert_eq!(test_sen_money_number(17300),-6);
    }
    #[test]
    fn t_17300_gosen_test(){
        assert_eq!(test_gosen_money_number(17300),-6);
    }
    #[test]
    fn t_17400_hyaku_test(){
        assert_eq!(test_hyaku_money_number(17400),0);
    }
    #[test]
    fn t_17400_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(17400),4);
    }
    #[test]
    fn t_17400_sen_test(){
        assert_eq!(test_sen_money_number(17400),-6);
    }
    #[test]
    fn t_17400_gosen_test(){
        assert_eq!(test_gosen_money_number(17400),-6);
    }
    #[test]
    fn t_17500_hyaku_test(){
        assert_eq!(test_hyaku_money_number(17500),0);
    }
    #[test]
    fn t_17500_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(17500),4);
    }
    #[test]
    fn t_17500_sen_test(){
        assert_eq!(test_sen_money_number(17500),-7);
    }
    #[test]
    fn t_17500_gosen_test(){
        assert_eq!(test_gosen_money_number(17500),-6);
    }
    #[test]
    fn t_17600_hyaku_test(){
        assert_eq!(test_hyaku_money_number(17600),25);
    }
    #[test]
    fn t_17600_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(17600),-3);
    }
    #[test]
    fn t_17600_sen_test(){
        assert_eq!(test_sen_money_number(17600),-7);
    }
    #[test]
    fn t_17600_gosen_test(){
        assert_eq!(test_gosen_money_number(17600),-6);
    }
    #[test]
    fn t_17700_hyaku_test(){
        assert_eq!(test_hyaku_money_number(17700),15);
    }
    #[test]
    fn t_17700_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(17700),-3);
    }
    #[test]
    fn t_17700_sen_test(){
        assert_eq!(test_sen_money_number(17700),-7);
    }
    #[test]
    fn t_17700_gosen_test(){
        assert_eq!(test_gosen_money_number(17700),-6);
    }
    #[test]
    fn t_17800_hyaku_test(){
        assert_eq!(test_hyaku_money_number(17800),5);
    }
    #[test]
    fn t_17800_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(17800),-3);
    }
    #[test]
    fn t_17800_sen_test(){
        assert_eq!(test_sen_money_number(17800),-7);
    }
    #[test]
    fn t_17800_gosen_test(){
        assert_eq!(test_gosen_money_number(17800),-6);
    }
    #[test]
    fn t_17900_hyaku_test(){
        assert_eq!(test_hyaku_money_number(17900),-5);
    }
    #[test]
    fn t_17900_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(17900),-3);
    }
    #[test]
    fn t_17900_sen_test(){
        assert_eq!(test_sen_money_number(17900),-7);
    }
    #[test]
    fn t_17900_gosen_test(){
        assert_eq!(test_gosen_money_number(17900),-6);
    }
    #[test]
    fn t_18000_hyaku_test(){
        assert_eq!(test_hyaku_money_number(18000),0);
    }
    #[test]
    fn t_18000_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(18000),0);
    }
    #[test]
    fn t_18000_sen_test(){
        assert_eq!(test_sen_money_number(18000),-5);
    }
    #[test]
    fn t_18000_gosen_test(){
        assert_eq!(test_gosen_money_number(18000),-5);
    }
    #[test]
    fn t_18100_hyaku_test(){
        assert_eq!(test_hyaku_money_number(18100),30);
    }
    #[test]
    fn t_18100_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(18100),4);
    }
    #[test]
    fn t_18100_sen_test(){
        assert_eq!(test_sen_money_number(18100),-16);
    }
    #[test]
    fn t_18100_gosen_test(){
        assert_eq!(test_gosen_money_number(18100),-6);
    }
    #[test]
    fn t_18200_hyaku_test(){
        assert_eq!(test_hyaku_money_number(18200),20);
    }
    #[test]
    fn t_18200_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(18200),4);
    }
    #[test]
    fn t_18200_sen_test(){
        assert_eq!(test_sen_money_number(18200),-16);
    }
    #[test]
    fn t_18200_gosen_test(){
        assert_eq!(test_gosen_money_number(18200),-6);
    }
    #[test]
    fn t_18300_hyaku_test(){
        assert_eq!(test_hyaku_money_number(18300),10);
    }
    #[test]
    fn t_18300_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(18300),4);
    }
    #[test]
    fn t_18300_sen_test(){
        assert_eq!(test_sen_money_number(18300),-16);
    }
    #[test]
    fn t_18300_gosen_test(){
        assert_eq!(test_gosen_money_number(18300),-6);
    }
    #[test]
    fn t_18400_hyaku_test(){
        assert_eq!(test_hyaku_money_number(18400),0);
    }
    #[test]
    fn t_18400_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(18400),4);
    }
    #[test]
    fn t_18400_sen_test(){
        assert_eq!(test_sen_money_number(18400),-16);
    }
    #[test]
    fn t_18400_gosen_test(){
        assert_eq!(test_gosen_money_number(18400),-6);
    }
    #[test]
    fn t_18500_hyaku_test(){
        assert_eq!(test_hyaku_money_number(18500),0);
    }
    #[test]
    fn t_18500_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(18500),4);
    }
    #[test]
    fn t_18500_sen_test(){
        assert_eq!(test_sen_money_number(18500),-17);
    }
    #[test]
    fn t_18500_gosen_test(){
        assert_eq!(test_gosen_money_number(18500),-6);
    }
    #[test]
    fn t_18600_hyaku_test(){
        assert_eq!(test_hyaku_money_number(18600),25);
    }
    #[test]
    fn t_18600_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(18600),-3);
    }
    #[test]
    fn t_18600_sen_test(){
        assert_eq!(test_sen_money_number(18600),-17);
    }
    #[test]
    fn t_18600_gosen_test(){
        assert_eq!(test_gosen_money_number(18600),-6);
    }
    #[test]
    fn t_18700_hyaku_test(){
        assert_eq!(test_hyaku_money_number(18700),15);
    }
    #[test]
    fn t_18700_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(18700),-3);
    }
    #[test]
    fn t_18700_sen_test(){
        assert_eq!(test_sen_money_number(18700),-17);
    }
    #[test]
    fn t_18700_gosen_test(){
        assert_eq!(test_gosen_money_number(18700),-6);
    }
    #[test]
    fn t_18800_hyaku_test(){
        assert_eq!(test_hyaku_money_number(18800),5);
    }
    #[test]
    fn t_18800_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(18800),-3);
    }
    #[test]
    fn t_18800_sen_test(){
        assert_eq!(test_sen_money_number(18800),-17);
    }
    #[test]
    fn t_18800_gosen_test(){
        assert_eq!(test_gosen_money_number(18800),-6);
    }
    #[test]
    fn t_18900_hyaku_test(){
        assert_eq!(test_hyaku_money_number(18900),-5);
    }
    #[test]
    fn t_18900_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(18900),-3);
    }
    #[test]
    fn t_18900_sen_test(){
        assert_eq!(test_sen_money_number(18900),-17);
    }
    #[test]
    fn t_18900_gosen_test(){
        assert_eq!(test_gosen_money_number(18900),-6);
    }
    #[test]
    fn t_19000_hyaku_test(){
        assert_eq!(test_hyaku_money_number(19000),0);
    }
    #[test]
    fn t_19000_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(19000),0);
    }
    #[test]
    fn t_19000_sen_test(){
        assert_eq!(test_sen_money_number(19000),-15);
    }
    #[test]
    fn t_19000_gosen_test(){
        assert_eq!(test_gosen_money_number(19000),-5);
    }
    #[test]
    fn t_19100_hyaku_test(){
        assert_eq!(test_hyaku_money_number(19100),25);
    }
    #[test]
    fn t_19100_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(19100),1);
    }
    #[test]
    fn t_19100_sen_test(){
        assert_eq!(test_sen_money_number(19100),-24);
    }
    #[test]
    fn t_19100_gosen_test(){
        assert_eq!(test_gosen_money_number(19100),-6);
    }
    #[test]
    fn t_19200_hyaku_test(){
        assert_eq!(test_hyaku_money_number(19200),15);
    }
    #[test]
    fn t_19200_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(19200),1);
    }
    #[test]
    fn t_19200_sen_test(){
        assert_eq!(test_sen_money_number(19200),-24);
    }
    #[test]
    fn t_19200_gosen_test(){
        assert_eq!(test_gosen_money_number(19200),-6);
    }
    #[test]
    fn t_19300_hyaku_test(){
        assert_eq!(test_hyaku_money_number(19300),5);
    }
    #[test]
    fn t_19300_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(19300),1);
    }
    #[test]
    fn t_19300_sen_test(){
        assert_eq!(test_sen_money_number(19300),-24);
    }
    #[test]
    fn t_19300_gosen_test(){
        assert_eq!(test_gosen_money_number(19300),-6);
    }
    #[test]
    fn t_19400_hyaku_test(){
        assert_eq!(test_hyaku_money_number(19400),-5);
    }
    #[test]
    fn t_19400_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(19400),1);
    }
    #[test]
    fn t_19400_sen_test(){
        assert_eq!(test_sen_money_number(19400),-24);
    }
    #[test]
    fn t_19400_gosen_test(){
        assert_eq!(test_gosen_money_number(19400),-6);
    }
    #[test]
    fn t_19500_hyaku_test(){
        assert_eq!(test_hyaku_money_number(19500),0);
    }
    #[test]
    fn t_19500_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(19500),0);
    }
    #[test]
    fn t_19500_sen_test(){
        assert_eq!(test_sen_money_number(19500),-20);
    }
    #[test]
    fn t_19500_gosen_test(){
        assert_eq!(test_gosen_money_number(19500),-5);
    }
    #[test]
    fn t_19600_hyaku_test(){
        assert_eq!(test_hyaku_money_number(19600),15);
    }
    #[test]
    fn t_19600_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(19600),-5);
    }
    #[test]
    fn t_19600_sen_test(){
        assert_eq!(test_sen_money_number(19600),-20);
    }
    #[test]
    fn t_19600_gosen_test(){
        assert_eq!(test_gosen_money_number(19600),-5);
    }
    #[test]
    fn t_19700_hyaku_test(){
        assert_eq!(test_hyaku_money_number(19700),5);
    }
    #[test]
    fn t_19700_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(19700),-5);
    }
    #[test]
    fn t_19700_sen_test(){
        assert_eq!(test_sen_money_number(19700),-20);
    }
    #[test]
    fn t_19700_gosen_test(){
        assert_eq!(test_gosen_money_number(19700),-5);
    }
    #[test]
    fn t_19800_hyaku_test(){
        assert_eq!(test_hyaku_money_number(19800),-5);
    }
    #[test]
    fn t_19800_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(19800),-5);
    }
    #[test]
    fn t_19800_sen_test(){
        assert_eq!(test_sen_money_number(19800),-20);
    }
    #[test]
    fn t_19800_gosen_test(){
        assert_eq!(test_gosen_money_number(19800),-5);
    }
    #[test]
    fn t_19900_hyaku_test(){
        assert_eq!(test_hyaku_money_number(19900),-15);
    }
    #[test]
    fn t_19900_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(19900),-5);
    }
    #[test]
    fn t_19900_sen_test(){
        assert_eq!(test_sen_money_number(19900),-20);
    }
    #[test]
    fn t_19900_gosen_test(){
        assert_eq!(test_gosen_money_number(19900),-5);
    }
    #[test]
    fn t_20000_hyaku_test(){
        assert_eq!(test_hyaku_money_number(20000),0);
    }
    #[test]
    fn t_20000_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(20000),0);
    }
    #[test]
    fn t_20000_sen_test(){
        assert_eq!(test_sen_money_number(20000),0);
    }
    #[test]
    fn t_20000_gosen_test(){
        assert_eq!(test_gosen_money_number(20000),0);
    }
    #[test]
    fn t_20100_hyaku_test(){
        assert_eq!(test_hyaku_money_number(20100),30);
    }
    #[test]
    fn t_20100_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(20100),4);
    }
    #[test]
    fn t_20100_sen_test(){
        assert_eq!(test_sen_money_number(20100),14);
    }
    #[test]
    fn t_20100_gosen_test(){
        assert_eq!(test_gosen_money_number(20100),-4);
    }
    #[test]
    fn t_20500_hyaku_test(){
        assert_eq!(test_hyaku_money_number(20500),0);
    }
    #[test]
    fn t_20500_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(20500),4);
    }
    #[test]
    fn t_20500_sen_test(){
        assert_eq!(test_sen_money_number(20500),13);
    }
    #[test]
    fn t_20500_gosen_test(){
        assert_eq!(test_gosen_money_number(20500),-4);
    }
    #[test]
    fn t_20900_hyaku_test(){
        assert_eq!(test_hyaku_money_number(20900),-5);
    }
    #[test]
    fn t_20900_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(20900),-3);
    }
    #[test]
    fn t_20900_sen_test(){
        assert_eq!(test_sen_money_number(20900),13);
    }
    #[test]
    fn t_20900_gosen_test(){
        assert_eq!(test_gosen_money_number(20900),-4);
    }
    #[test]
    fn t_21000_hyaku_test(){
        assert_eq!(test_hyaku_money_number(21000),0);
    }
    #[test]
    fn t_21000_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(21000),0);
    }
    #[test]
    fn t_21000_sen_test(){
        assert_eq!(test_sen_money_number(21000),15);
    }
    #[test]
    fn t_21000_gosen_test(){
        assert_eq!(test_gosen_money_number(21000),-5);
    }
    #[test]
    fn t_21100_hyaku_test(){
        assert_eq!(test_hyaku_money_number(21100),30);
    }
    #[test]
    fn t_21100_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(21100),4);
    }
    #[test]
    fn t_21100_sen_test(){
        assert_eq!(test_sen_money_number(21100),4);
    }
    #[test]
    fn t_21100_gosen_test(){
        assert_eq!(test_gosen_money_number(21100),-4);
    }
    #[test]
    fn t_21500_hyaku_test(){
        assert_eq!(test_hyaku_money_number(21500),0);
    }
    #[test]
    fn t_21500_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(21500),4);
    }
    #[test]
    fn t_21500_sen_test(){
        assert_eq!(test_sen_money_number(21500),3);
    }
    #[test]
    fn t_21500_gosen_test(){
        assert_eq!(test_gosen_money_number(21500),-4);
    }
    #[test]
    fn t_21900_hyaku_test(){
        assert_eq!(test_hyaku_money_number(21900),-5);
    }
    #[test]
    fn t_21900_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(21900),-3);
    }
    #[test]
    fn t_21900_sen_test(){
        assert_eq!(test_sen_money_number(21900),3);
    }
    #[test]
    fn t_21900_gosen_test(){
        assert_eq!(test_gosen_money_number(21900),-4);
    }
    #[test]
    fn t_22000_hyaku_test(){
        assert_eq!(test_hyaku_money_number(22000),0);
    }
    #[test]
    fn t_22000_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(22000),0);
    }
    #[test]
    fn t_22000_sen_test(){
        assert_eq!(test_sen_money_number(22000),5);
    }
    #[test]
    fn t_22000_gosen_test(){
        assert_eq!(test_gosen_money_number(22000),-5);
    }
    #[test]
    fn t_22100_hyaku_test(){
        assert_eq!(test_hyaku_money_number(22100),30);
    }
    #[test]
    fn t_22100_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(22100),4);
    }
    #[test]
    fn t_22100_sen_test(){
        assert_eq!(test_sen_money_number(22100),-6);
    }
    #[test]
    fn t_22100_gosen_test(){
        assert_eq!(test_gosen_money_number(22100),-4);
    }
    #[test]
    fn t_22500_hyaku_test(){
        assert_eq!(test_hyaku_money_number(22500),0);
    }
    #[test]
    fn t_22500_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(22500),4);
    }
    #[test]
    fn t_22500_sen_test(){
        assert_eq!(test_sen_money_number(22500),-7);
    }
    #[test]
    fn t_22500_gosen_test(){
        assert_eq!(test_gosen_money_number(22500),-4);
    }
    #[test]
    fn t_22900_hyaku_test(){
        assert_eq!(test_hyaku_money_number(22900),-5);
    }
    #[test]
    fn t_22900_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(22900),-3);
    }
    #[test]
    fn t_22900_sen_test(){
        assert_eq!(test_sen_money_number(22900),-7);
    }
    #[test]
    fn t_22900_gosen_test(){
        assert_eq!(test_gosen_money_number(22900),-4);
    }
    #[test]
    fn t_23000_hyaku_test(){
        assert_eq!(test_hyaku_money_number(23000),0);
    }
    #[test]
    fn t_23000_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(23000),0);
    }
    #[test]
    fn t_23000_sen_test(){
        assert_eq!(test_sen_money_number(23000),-5);
    }
    #[test]
    fn t_23000_gosen_test(){
        assert_eq!(test_gosen_money_number(23000),-5);
    }
    #[test]
    fn t_23100_hyaku_test(){
        assert_eq!(test_hyaku_money_number(23100),30);
    }
    #[test]
    fn t_23100_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(23100),4);
    }
    #[test]
    fn t_23100_sen_test(){
        assert_eq!(test_sen_money_number(23100),-16);
    }
    #[test]
    fn t_23100_gosen_test(){
        assert_eq!(test_gosen_money_number(23100),-4);
    }
    #[test]
    fn t_23500_hyaku_test(){
        assert_eq!(test_hyaku_money_number(23500),0);
    }
    #[test]
    fn t_23500_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(23500),4);
    }
    #[test]
    fn t_23500_sen_test(){
        assert_eq!(test_sen_money_number(23500),-17);
    }
    #[test]
    fn t_23500_gosen_test(){
        assert_eq!(test_gosen_money_number(23500),-4);
    }
    #[test]
    fn t_23900_hyaku_test(){
        assert_eq!(test_hyaku_money_number(23900),-5);
    }
    #[test]
    fn t_23900_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(23900),-3);
    }
    #[test]
    fn t_23900_sen_test(){
        assert_eq!(test_sen_money_number(23900),-17);
    }
    #[test]
    fn t_23900_gosen_test(){
        assert_eq!(test_gosen_money_number(23900),-4);
    }
    #[test]
    fn t_24000_hyaku_test(){
        assert_eq!(test_hyaku_money_number(24000),0);
    }
    #[test]
    fn t_24000_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(24000),0);
    }
    #[test]
    fn t_24000_sen_test(){
        assert_eq!(test_sen_money_number(24000),-15);
    }
    #[test]
    fn t_24000_gosen_test(){
        assert_eq!(test_gosen_money_number(24000),-5);
    }
    #[test]
    fn t_24100_hyaku_test(){
        assert_eq!(test_hyaku_money_number(24100),25);
    }
    #[test]
    fn t_24100_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(24100),1);
    }
    #[test]
    fn t_24100_sen_test(){
        assert_eq!(test_sen_money_number(24100),-24);
    }
    #[test]
    fn t_24100_gosen_test(){
        assert_eq!(test_gosen_money_number(24100),-4);
    }
    #[test]
    fn t_24500_hyaku_test(){
        assert_eq!(test_hyaku_money_number(24500),0);
    }
    #[test]
    fn t_24500_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(24500),0);
    }
    #[test]
    fn t_24500_sen_test(){
        assert_eq!(test_sen_money_number(24500),-20);
    }
    #[test]
    fn t_24500_gosen_test(){
        assert_eq!(test_gosen_money_number(24500),-5);
    }
    #[test]
    fn t_25000_hyaku_test(){
        assert_eq!(test_hyaku_money_number(25000),0);
    }
    #[test]
    fn t_25000_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(25000),0);
    }
    #[test]
    fn t_24900_hyaku_test(){
        assert_eq!(test_hyaku_money_number(24900),-15);
    }
    #[test]
    fn t_24900_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(24900),-5);
    }
    #[test]
    fn t_24900_sen_test(){
        assert_eq!(test_sen_money_number(24900),-20);
    }
    #[test]
    fn t_24900_gosen_test(){
        assert_eq!(test_gosen_money_number(24900),-5);
    }
    #[test]
    fn t_25000_sen_test(){
        assert_eq!(test_sen_money_number(25000),0);
    }
    #[test]
    fn t_25000_gosen_test(){
        assert_eq!(test_gosen_money_number(25000),0);
    }
    #[test]
    fn t_25100_hyaku_test(){
        assert_eq!(test_hyaku_money_number(25100),30);
    }
    #[test]
    fn t_25100_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(25100),4);
    }
    #[test]
    fn t_25100_sen_test(){
        assert_eq!(test_sen_money_number(25100),14);
    }
    #[test]
    fn t_25100_gosen_test(){
        assert_eq!(test_gosen_money_number(25100),-6);
    }
    #[test]
    fn t_25500_hyaku_test(){
        assert_eq!(test_hyaku_money_number(25500),0);
    }
    #[test]
    fn t_25500_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(25500),4);
    }
    #[test]
    fn t_25500_sen_test(){
        assert_eq!(test_sen_money_number(25500),13);
    }
    #[test]
    fn t_25500_gosen_test(){
        assert_eq!(test_gosen_money_number(25500),-6);
    }
    #[test]
    fn t_25900_hyaku_test(){
        assert_eq!(test_hyaku_money_number(25900),-5);
    }
    #[test]
    fn t_25900_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(25800),-3);
    }
    #[test]
    fn t_25900_sen_test(){
        assert_eq!(test_sen_money_number(25800),13);
    }
    #[test]
    fn t_25900_gosen_test(){
        assert_eq!(test_gosen_money_number(25800),-6);
    }
    #[test]
    fn t_26000_hyaku_test(){
        assert_eq!(test_hyaku_money_number(26000),0);
    }
    #[test]
    fn t_26000_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(26000),0);
    }
    #[test]
    fn t_26000_sen_test(){
        assert_eq!(test_sen_money_number(26000),15);
    }
    #[test]
    fn t_26000_gosen_test(){
        assert_eq!(test_gosen_money_number(26000),-5);
    }
    #[test]
    fn t_26100_hyaku_test(){
        assert_eq!(test_hyaku_money_number(26100),30);
    }
    #[test]
    fn t_26100_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(26100),4);
    }
    #[test]
    fn t_26100_sen_test(){
        assert_eq!(test_sen_money_number(26100),4);
    }
    #[test]
    fn t_26100_gosen_test(){
        assert_eq!(test_gosen_money_number(26100),-6);
    }
    #[test]
    fn t_26500_hyaku_test(){
        assert_eq!(test_hyaku_money_number(26500),0);
    }
    #[test]
    fn t_26500_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(26500),4);
    }
    #[test]
    fn t_26500_sen_test(){
        assert_eq!(test_sen_money_number(26500),3);
    }
    #[test]
    fn t_26500_gosen_test(){
        assert_eq!(test_gosen_money_number(26500),-6);
    }
    #[test]
    fn t_26900_hyaku_test(){
        assert_eq!(test_hyaku_money_number(26900),-5);
    }
    #[test]
    fn t_26900_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(26900),-3);
    }
    #[test]
    fn t_26900_sen_test(){
        assert_eq!(test_sen_money_number(26900),3);
    }
    #[test]
    fn t_26900_gosen_test(){
        assert_eq!(test_gosen_money_number(26900),-6);
    }
    #[test]
    fn t_27000_hyaku_test(){
        assert_eq!(test_hyaku_money_number(27000),0);
    }
    #[test]
    fn t_27000_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(27000),0);
    }
    #[test]
    fn t_27000_sen_test(){
        assert_eq!(test_sen_money_number(27000),5);
    }
    #[test]
    fn t_27000_gosen_test(){
        assert_eq!(test_gosen_money_number(27000),-5);
    }
    #[test]
    fn t_27100_hyaku_test(){
        assert_eq!(test_hyaku_money_number(27100),30);
    }
    #[test]
    fn t_27100_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(27100),4);
    }
    #[test]
    fn t_27100_sen_test(){
        assert_eq!(test_sen_money_number(27100),-6);
    }
    #[test]
    fn t_27100_gosen_test(){
        assert_eq!(test_gosen_money_number(27100),-6);
    }
    #[test]
    fn t_27500_hyaku_test(){
        assert_eq!(test_hyaku_money_number(27500),0);
    }
    #[test]
    fn t_27500_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(27500),4);
    }
    #[test]
    fn t_27500_sen_test(){
        assert_eq!(test_sen_money_number(27500),-7);
    }
    #[test]
    fn t_27500_gosen_test(){
        assert_eq!(test_gosen_money_number(27500),-6);
    }
    #[test]
    fn t_27900_hyaku_test(){
        assert_eq!(test_hyaku_money_number(27900),-5);
    }
    #[test]
    fn t_27900_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(27900),-3);
    }
    #[test]
    fn t_27900_sen_test(){
        assert_eq!(test_sen_money_number(27900),-7);
    }
    #[test]
    fn t_27900_gosen_test(){
        assert_eq!(test_gosen_money_number(27900),-6);
    }
    #[test]
    fn t_28000_hyaku_test(){
        assert_eq!(test_hyaku_money_number(28000),0);
    }
    #[test]
    fn t_28000_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(28000),0);
    }
    #[test]
    fn t_28000_sen_test(){
        assert_eq!(test_sen_money_number(28000),-5);
    }
    #[test]
    fn t_28000_gosen_test(){
        assert_eq!(test_gosen_money_number(28000),-5);
    }
    #[test]
    fn t_28100_hyaku_test(){
        assert_eq!(test_hyaku_money_number(28100),30);
    }
    #[test]
    fn t_28100_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(28100),4);
    }
    #[test]
    fn t_28100_sen_test(){
        assert_eq!(test_sen_money_number(28100),-16);
    }
    #[test]
    fn t_28500_hyaku_test(){
        assert_eq!(test_hyaku_money_number(28500),0);
    }
    #[test]
    fn t_28500_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(28500),4);
    }
    #[test]
    fn t_28500_sen_test(){
        assert_eq!(test_sen_money_number(28500),-17);
    }
    #[test]
    fn t_28500_gosen_test(){
        assert_eq!(test_gosen_money_number(28500),-6);
    }
    #[test]
    fn t_28900_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(28900),-3);
    }
    #[test]
    fn t_28900_sen_test(){
        assert_eq!(test_sen_money_number(28900),-17);
    }
    #[test]
    fn t_28900_gosen_test(){
        assert_eq!(test_gosen_money_number(28900),-6);
    }
    #[test]
    fn t_29000_hyaku_test(){
        assert_eq!(test_hyaku_money_number(29000),0);
    }
    #[test]
    fn t_29000_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(29000),0);
    }
    #[test]
    fn t_29000_sen_test(){
        assert_eq!(test_sen_money_number(29000),-15);
    }
    #[test]
    fn t_29000_gosen_test(){
        assert_eq!(test_gosen_money_number(29000),-5);
    }
    #[test]
    fn t_29100_hyaku_test(){
        assert_eq!(test_hyaku_money_number(29100),25);
    }
    #[test]
    fn t_29100_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(29100),1);
    }
    #[test]
    fn t_29500_hyaku_test(){
        assert_eq!(test_hyaku_money_number(29500),0);
    }
    #[test]
    fn t_29500_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(29500),0);
    }
    #[test]
    fn t_29500_sen_test(){
        assert_eq!(test_sen_money_number(29500),-20);
    }
    #[test]
    fn t_29500_gosen_test(){
        assert_eq!(test_gosen_money_number(29500),-5);
    }
    #[test]
    fn t_29900_hyaku_test(){
        assert_eq!(test_hyaku_money_number(29900),-15);
    }
    #[test]
    fn t_29900_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(29900),-5);
    }
    #[test]
    fn t_29900_sen_test(){
        assert_eq!(test_sen_money_number(29900),-20);
    }
    #[test]
    fn t_29900_gosen_test(){
        assert_eq!(test_gosen_money_number(29900),-5);
    }
    #[test]
    fn t_30000_hyaku_test(){
        assert_eq!(test_hyaku_money_number(30000),0);
    }
    #[test]
    fn t_30000_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(30000),0);
    }
    #[test]
    fn t_30000_sen_test(){
        assert_eq!(test_sen_money_number(30000),0);
    }
    #[test]
    fn t_30000_gosen_test(){
        assert_eq!(test_gosen_money_number(30000),0);
    }
    #[test]
    fn t_30100_hyaku_test(){
        assert_eq!(test_hyaku_money_number(30100),30);
    }
    #[test]
    fn t_30100_gohyaku_test(){
        assert_eq!(test_gohyaku_money_number(30100),4);
    }
    #[test]
    fn t_30100_sen_test(){
        assert_eq!(test_sen_money_number(30100),14);
    }
    #[test]
    fn t_30100_gosen_test(){
        assert_eq!(test_gosen_money_number(30100),-4);
    }
    #[test]
    fn t_comb_hyaku_test(){
        assert_eq!(test_combination_hyaku(),40);
    }
    #[test]
    fn t_comb_gohyaku_test(){
        assert_eq!(test_combination_gohyaku(),4);
    }
    #[test]
    fn t_comb_sen_test(){
        assert_eq!(test_combination_sen(),-21);
    }
    #[test]
    fn t_comb_gosen_test(){
        // 10 : 0
        // 50 : 0
        // 100 : 40
        // 500 : 4
        // 1000 : 0
        // 5000 : 0
        assert_eq!(test_combination_gosen(),0);
    }
    #[test]
    fn t_comb_ju2_test(){
        assert_eq!(test_combination_ju2(),145);
    }
    #[test]
    fn t_comb_goju2_test(){
        assert_eq!(test_combination_goju2(),23);
    }
    #[test]
    fn t_comb_hyaku2_test(){
        assert_eq!(test_combination_hyaku2(),-7);
    }
   #[test]
    fn t_comb_gohyaku2_test(){
        assert_eq!(test_combination_gohyaku2(),-7);
    }
    #[test]
    fn t_comb_sen2_test(){
        assert_eq!(test_combination_sen2(),-95);
    }
    #[test]
    fn t_comb_gosen2_test(){
        // 10 : 0
        // 50 : 0
        // 100 : 40
        // 500 : 4
        // 1000 : 0
        // 5000 : 0
        assert_eq!(test_combination_gosen2(),0);
    }
    #[test]
    fn t_comb_ju3_test(){
        assert_eq!(test_combination_ju3(),0);
    }
    #[test]
    fn t_comb_goju3_test(){
        assert_eq!(test_combination_goju3(),0);
    }
    #[test]
    fn t_comb_hyaku3_test(){
        assert_eq!(test_combination_hyaku3(),4595);
    }
   #[test]
    fn t_comb_gohyaku3_test(){
        assert_eq!(test_combination_gohyaku3(),-99);
    }
    #[test]
    fn t_comb_sen3_test(){
        assert_eq!(test_combination_sen3(),-2733);
    }
    #[test]
    fn t_comb_gosen3_test(){
        assert_eq!(test_combination_gosen3(),-605);
    }
}

#[wasm_bindgen]
pub fn calc(){
    let hon_from_js_list: Vec<JsValue>=getHonList().to_vec();
    if hon_from_js_list.len()==0 {
        return;
    }
    let mut hon_list: Vec<Hon>=Vec::new();
    let mut id: i32=0;
    for hon_from_js in hon_from_js_list.iter(){
        let hon_tmp: Option<String>=hon_from_js.as_string();
        let hon_str: String=hon_tmp.unwrap();
        let hon_split_list: Vec<&str>=hon_str.split(":").collect();
        let price_str: &str=hon_split_list[0];
        let hanpusu_str: &str=hon_split_list[1];
        let price_int: i32=price_str.parse().unwrap();
        let hanpusu_int: i32=hanpusu_str.parse().unwrap();
        let hon = Hon{
            id: id,
            price: price_int,
            hanpusu: hanpusu_int,
            hanpu_count: 0,
            hanpu_number_combination: 0,
            amari: 0
        };
        hon_list.push(hon);
        id+=1;
    }
    let mut result_for_screen=MoneyNumber{
        ju: 0,
        goju: 0,
        hyaku: 0,
        gohyaku: 0,
        sen: 0,
        gosen: 0,
        ichiman: 0
    };
    let ret_money_number=MoneyNumber{
        ju: 0,
        goju: 0,
        hyaku: 0,
        gohyaku: 0,
        sen: 0,
        gosen: 0,
        ichiman: 0
    };
    let mut ret=CalcResult{
        money_number: ret_money_number,
        hon_list: Vec::new(),
        end_flag: false
    }; 
    // 組み合わせを列挙してお釣りを計算します。
    while !ret.end_flag {
        ret=calc_combination(hon_list);
        result_for_screen.ju+=ret.money_number.ju;
        result_for_screen.goju+=ret.money_number.goju;
        result_for_screen.hyaku+=ret.money_number.hyaku;
        result_for_screen.gohyaku+=ret.money_number.gohyaku;
        result_for_screen.sen+=ret.money_number.sen;
        result_for_screen.gosen+=ret.money_number.gosen;
        result_for_screen.ichiman+=ret.money_number.ichiman;
        hon_list=ret.hon_list;
    }
    setResult(result_for_screen.ju,result_for_screen.goju,result_for_screen.hyaku,
        result_for_screen.gohyaku,result_for_screen.sen,result_for_screen.gosen);
}

pub fn test_ju_money_number(price: i32)->i32{
    let ret: MoneyNumber=calc_ikura(price,10);
    return ret.ju;
}

pub fn test_goju_money_number(price: i32)->i32{
    let ret: MoneyNumber=calc_ikura(price,10);
    return ret.goju;
}

pub fn test_hyaku_money_number(price: i32)->i32{
    let ret: MoneyNumber=calc_ikura(price,10);
    return ret.hyaku;
}

pub fn test_gohyaku_money_number(price: i32)->i32{
    let ret: MoneyNumber=calc_ikura(price,10);
    return ret.gohyaku;
}

pub fn test_sen_money_number(price: i32)->i32{
    let ret: MoneyNumber=calc_ikura(price,10);
    return ret.sen;
}

pub fn test_gosen_money_number(price: i32)->i32{
    let ret: MoneyNumber=calc_ikura(price,10);
    return ret.gosen;
}

pub fn test_ichiman_money_number(price: i32)->i32{
    let ret: MoneyNumber=calc_ikura(price,10);
    return ret.ichiman;
}

pub fn test_combination_ju()->i32{
    // 1200 10
    // 600  10
    // 400  5
    // 500  10    
    // TODO : juとgojuがテスト出来るように。
    // 10 : 0
    // 50 : 0
    // 100 : 40
    // 500 : 4
    // 1000 : 0
    // 5000 : 0
    let hon1 = Hon{
        id: 1,
        price: 1200,
        hanpusu: 10,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon2 = Hon{
        id: 2,
        price: 600,
        hanpusu: 10,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon3 = Hon{
        id: 3,
        price: 400,
        hanpusu: 5,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon4 = Hon{
        id: 4,
        price: 500,
        hanpusu: 10,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let mut hon_list=Vec::new();
    hon_list.push(hon1);
    hon_list.push(hon2);
    hon_list.push(hon3);
    hon_list.push(hon4);

    let ret_money_number=MoneyNumber{
        ju: 0,
        goju: 0,
        hyaku: 0,
        gohyaku: 0,
        sen: 0,
        gosen: 0,
        ichiman: 0
    };
    let mut result=MoneyNumber{
        ju: 0,
        goju: 0,
        hyaku: 0,
        gohyaku: 0,
        sen: 0,
        gosen: 0,
        ichiman: 0
    };
    let mut ret=CalcResult{
        money_number: ret_money_number,
        hon_list: Vec::new(),
        end_flag: false
    }; 
    while !ret.end_flag {
        ret=calc_combination(hon_list);
        result.ju+=ret.money_number.ju;
        result.goju+=ret.money_number.goju;
        result.hyaku+=ret.money_number.hyaku;
        result.gohyaku+=ret.money_number.gohyaku;
        result.sen+=ret.money_number.sen;
        result.gosen+=ret.money_number.gosen;
        result.ichiman+=ret.money_number.ichiman;
        hon_list=ret.hon_list;
    }
    return result.ju;
}

pub fn test_combination_goju()->i32{
    // 1200 10
    // 600  10
    // 400  5
    // 500  10    
    // TODO : juとgojuがテスト出来るように。
    // 10 : 0
    // 50 : 0
    // 100 : 40
    // 500 : 4
    // 1000 : 0
    // 5000 : 0
    let hon1 = Hon{
        id: 1,
        price: 1200,
        hanpusu: 10,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon2 = Hon{
        id: 2,
        price: 600,
        hanpusu: 10,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon3 = Hon{
        id: 3,
        price: 400,
        hanpusu: 5,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon4 = Hon{
        id: 4,
        price: 500,
        hanpusu: 10,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let mut hon_list=Vec::new();
    hon_list.push(hon1);
    hon_list.push(hon2);
    hon_list.push(hon3);
    hon_list.push(hon4);

    let ret_money_number=MoneyNumber{
        ju: 0,
        goju: 0,
        hyaku: 0,
        gohyaku: 0,
        sen: 0,
        gosen: 0,
        ichiman: 0
    };
    let mut result=MoneyNumber{
        ju: 0,
        goju: 0,
        hyaku: 0,
        gohyaku: 0,
        sen: 0,
        gosen: 0,
        ichiman: 0
    };
    let mut ret=CalcResult{
        money_number: ret_money_number,
        hon_list: Vec::new(),
        end_flag: false
    }; 
    while !ret.end_flag {
        ret=calc_combination(hon_list);
        result.ju+=ret.money_number.ju;
        result.goju+=ret.money_number.goju;
        result.hyaku+=ret.money_number.hyaku;
        result.gohyaku+=ret.money_number.gohyaku;
        result.sen+=ret.money_number.sen;
        result.gosen+=ret.money_number.gosen;
        result.ichiman+=ret.money_number.ichiman;
        hon_list=ret.hon_list;
    }
    return result.goju;
}

pub fn test_combination_hyaku()->i32{
    // 1200 10
    // 600  10
    // 400  5
    // 500  10    
    // TODO : juとgojuがテスト出来るように。
    // 10 : 0
    // 50 : 0
    // 100 : 40
    // 500 : 4
    // 1000 : 0
    // 5000 : 0
    let hon1 = Hon{
        id: 1,
        price: 1200,
        hanpusu: 10,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon2 = Hon{
        id: 2,
        price: 600,
        hanpusu: 10,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon3 = Hon{
        id: 3,
        price: 400,
        hanpusu: 5,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon4 = Hon{
        id: 4,
        price: 500,
        hanpusu: 10,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let mut hon_list=Vec::new();
    hon_list.push(hon1);
    hon_list.push(hon2);
    hon_list.push(hon3);
    hon_list.push(hon4);

    let ret_money_number=MoneyNumber{
        ju: 0,
        goju: 0,
        hyaku: 0,
        gohyaku: 0,
        sen: 0,
        gosen: 0,
        ichiman: 0
    };
    let mut result=MoneyNumber{
        ju: 0,
        goju: 0,
        hyaku: 0,
        gohyaku: 0,
        sen: 0,
        gosen: 0,
        ichiman: 0
    };
    let mut ret=CalcResult{
        money_number: ret_money_number,
        hon_list: Vec::new(),
        end_flag: false
    }; 
    while !ret.end_flag {
        ret=calc_combination(hon_list);
        result.ju+=ret.money_number.ju;
        result.goju+=ret.money_number.goju;
        result.hyaku+=ret.money_number.hyaku;
        result.gohyaku+=ret.money_number.gohyaku;
        result.sen+=ret.money_number.sen;
        result.gosen+=ret.money_number.gosen;
        result.ichiman+=ret.money_number.ichiman;
        hon_list=ret.hon_list;
    }
    return result.hyaku;
}

pub fn test_combination_gohyaku()->i32{
    // 1200 10
    // 600  10
    // 400  5
    // 500  10    
    // TODO : juとgojuがテスト出来るように。
    // 10 : 0
    // 50 : 0
    // 100 : 40
    // 500 : 4
    // 1000 : 0
    // 5000 : 0
    let hon1 = Hon{
        id: 1,
        price: 1200,
        hanpusu: 10,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon2 = Hon{
        id: 2,
        price: 600,
        hanpusu: 10,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon3 = Hon{
        id: 3,
        price: 400,
        hanpusu: 5,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon4 = Hon{
        id: 4,
        price: 500,
        hanpusu: 10,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let mut hon_list=Vec::new();
    hon_list.push(hon1);
    hon_list.push(hon2);
    hon_list.push(hon3);
    hon_list.push(hon4);

    let ret_money_number=MoneyNumber{
        ju: 0,
        goju: 0,
        hyaku: 0,
        gohyaku: 0,
        sen: 0,
        gosen: 0,
        ichiman: 0
    };
    let mut result=MoneyNumber{
        ju: 0,
        goju: 0,
        hyaku: 0,
        gohyaku: 0,
        sen: 0,
        gosen: 0,
        ichiman: 0
    };
    let mut ret=CalcResult{
        money_number: ret_money_number,
        hon_list: Vec::new(),
        end_flag: false
    }; 
    while !ret.end_flag {
        ret=calc_combination(hon_list);
        result.ju+=ret.money_number.ju;
        result.goju+=ret.money_number.goju;
        result.hyaku+=ret.money_number.hyaku;
        result.gohyaku+=ret.money_number.gohyaku;
        result.sen+=ret.money_number.sen;
        result.gosen+=ret.money_number.gosen;
        result.ichiman+=ret.money_number.ichiman;
        hon_list=ret.hon_list;
    }
    return result.gohyaku;
}

pub fn test_combination_sen()->i32{
    // 1200 10
    // 600  10
    // 400  5
    // 500  10    
    // TODO : juとgojuがテスト出来るように。
    // 10 : 0
    // 50 : 0
    // 100 : 40
    // 500 : 4
    // 1000 : 0
    // 5000 : 0
    let hon1 = Hon{
        id: 1,
        price: 1200,
        hanpusu: 10,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon2 = Hon{
        id: 2,
        price: 600,
        hanpusu: 10,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon3 = Hon{
        id: 3,
        price: 400,
        hanpusu: 5,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon4 = Hon{
        id: 4,
        price: 500,
        hanpusu: 10,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let mut hon_list=Vec::new();
    hon_list.push(hon1);
    hon_list.push(hon2);
    hon_list.push(hon3);
    hon_list.push(hon4);

    let ret_money_number=MoneyNumber{
        ju: 0,
        goju: 0,
        hyaku: 0,
        gohyaku: 0,
        sen: 0,
        gosen: 0,
        ichiman: 0
    };
    let mut result=MoneyNumber{
        ju: 0,
        goju: 0,
        hyaku: 0,
        gohyaku: 0,
        sen: 0,
        gosen: 0,
        ichiman: 0
    };
    let mut ret=CalcResult{
        money_number: ret_money_number,
        hon_list: Vec::new(),
        end_flag: false
    }; 
    while !ret.end_flag {
        ret=calc_combination(hon_list);
        result.ju+=ret.money_number.ju;
        result.goju+=ret.money_number.goju;
        result.hyaku+=ret.money_number.hyaku;
        result.gohyaku+=ret.money_number.gohyaku;
        result.sen+=ret.money_number.sen;
        result.gosen+=ret.money_number.gosen;
        result.ichiman+=ret.money_number.ichiman;
        hon_list=ret.hon_list;
    }
    return result.sen;
}

pub fn test_combination_gosen()->i32{
    // 1200 10
    // 600  10
    // 400  5
    // 500  10    
    // TODO : juとgojuがテスト出来るように。
    // 10 : 0
    // 50 : 0
    // 100 : 40
    // 500 : 4
    // 1000 : 0
    // 5000 : 0
    let hon1 = Hon{
        id: 1,
        price: 1200,
        hanpusu: 10,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon2 = Hon{
        id: 2,
        price: 600,
        hanpusu: 10,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon3 = Hon{
        id: 3,
        price: 400,
        hanpusu: 5,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon4 = Hon{
        id: 4,
        price: 500,
        hanpusu: 10,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let mut hon_list=Vec::new();
    hon_list.push(hon1);
    hon_list.push(hon2);
    hon_list.push(hon3);
    hon_list.push(hon4);

    let ret_money_number=MoneyNumber{
        ju: 0,
        goju: 0,
        hyaku: 0,
        gohyaku: 0,
        sen: 0,
        gosen: 0,
        ichiman: 0
    };
    let mut result=MoneyNumber{
        ju: 0,
        goju: 0,
        hyaku: 0,
        gohyaku: 0,
        sen: 0,
        gosen: 0,
        ichiman: 0
    };
    let mut ret=CalcResult{
        money_number: ret_money_number,
        hon_list: Vec::new(),
        end_flag: false
    }; 
    while !ret.end_flag {
        ret=calc_combination(hon_list);
        result.ju+=ret.money_number.ju;
        result.goju+=ret.money_number.goju;
        result.hyaku+=ret.money_number.hyaku;
        result.gohyaku+=ret.money_number.gohyaku;
        result.sen+=ret.money_number.sen;
        result.gosen+=ret.money_number.gosen;
        result.ichiman+=ret.money_number.ichiman;
        hon_list=ret.hon_list;
    }
    return result.gosen;
}

pub fn test_combination_ju2()->i32{
    let hon1 = Hon{
        id: 1,
        price: 110,
        hanpusu: 100,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon2 = Hon{
        id: 2,
        price: 460,
        hanpusu: 100,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon3 = Hon{
        id: 3,
        price: 740,
        hanpusu: 40,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon4 = Hon{
        id: 4,
        price: 880,
        hanpusu: 100,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let mut hon_list=Vec::new();
    hon_list.push(hon1);
    hon_list.push(hon2);
    hon_list.push(hon3);
    hon_list.push(hon4);

    let ret_money_number=MoneyNumber{
        ju: 0,
        goju: 0,
        hyaku: 0,
        gohyaku: 0,
        sen: 0,
        gosen: 0,
        ichiman: 0
    };
    let mut result=MoneyNumber{
        ju: 0,
        goju: 0,
        hyaku: 0,
        gohyaku: 0,
        sen: 0,
        gosen: 0,
        ichiman: 0
    };
    let mut ret=CalcResult{
        money_number: ret_money_number,
        hon_list: Vec::new(),
        end_flag: false
    }; 
    while !ret.end_flag {
        ret=calc_combination(hon_list);
        result.ju+=ret.money_number.ju;
        result.goju+=ret.money_number.goju;
        result.hyaku+=ret.money_number.hyaku;
        result.gohyaku+=ret.money_number.gohyaku;
        result.sen+=ret.money_number.sen;
        result.gosen+=ret.money_number.gosen;
        result.ichiman+=ret.money_number.ichiman;
        hon_list=ret.hon_list;
    }
    return result.ju;
}

pub fn test_combination_goju2()->i32{
    let hon1 = Hon{
        id: 1,
        price: 110,
        hanpusu: 100,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon2 = Hon{
        id: 2,
        price: 460,
        hanpusu: 100,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon3 = Hon{
        id: 3,
        price: 740,
        hanpusu: 40,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon4 = Hon{
        id: 4,
        price: 880,
        hanpusu: 100,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let mut hon_list=Vec::new();
    hon_list.push(hon1);
    hon_list.push(hon2);
    hon_list.push(hon3);
    hon_list.push(hon4);

    let ret_money_number=MoneyNumber{
        ju: 0,
        goju: 0,
        hyaku: 0,
        gohyaku: 0,
        sen: 0,
        gosen: 0,
        ichiman: 0
    };
    let mut result=MoneyNumber{
        ju: 0,
        goju: 0,
        hyaku: 0,
        gohyaku: 0,
        sen: 0,
        gosen: 0,
        ichiman: 0
    };
    let mut ret=CalcResult{
        money_number: ret_money_number,
        hon_list: Vec::new(),
        end_flag: false
    }; 
    while !ret.end_flag {
        ret=calc_combination(hon_list);
        result.ju+=ret.money_number.ju;
        result.goju+=ret.money_number.goju;
        result.hyaku+=ret.money_number.hyaku;
        result.gohyaku+=ret.money_number.gohyaku;
        result.sen+=ret.money_number.sen;
        result.gosen+=ret.money_number.gosen;
        result.ichiman+=ret.money_number.ichiman;
        hon_list=ret.hon_list;
    }
    return result.goju;
}

pub fn test_combination_hyaku2()->i32{
    let hon1 = Hon{
        id: 1,
        price: 110,
        hanpusu: 100,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon2 = Hon{
        id: 2,
        price: 460,
        hanpusu: 100,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon3 = Hon{
        id: 3,
        price: 740,
        hanpusu: 40,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon4 = Hon{
        id: 4,
        price: 880,
        hanpusu: 100,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let mut hon_list=Vec::new();
    hon_list.push(hon1);
    hon_list.push(hon2);
    hon_list.push(hon3);
    hon_list.push(hon4);

    let ret_money_number=MoneyNumber{
        ju: 0,
        goju: 0,
        hyaku: 0,
        gohyaku: 0,
        sen: 0,
        gosen: 0,
        ichiman: 0
    };
    let mut result=MoneyNumber{
        ju: 0,
        goju: 0,
        hyaku: 0,
        gohyaku: 0,
        sen: 0,
        gosen: 0,
        ichiman: 0
    };
    let mut ret=CalcResult{
        money_number: ret_money_number,
        hon_list: Vec::new(),
        end_flag: false
    }; 
    while !ret.end_flag {
        ret=calc_combination(hon_list);
        result.ju+=ret.money_number.ju;
        result.goju+=ret.money_number.goju;
        result.hyaku+=ret.money_number.hyaku;
        result.gohyaku+=ret.money_number.gohyaku;
        result.sen+=ret.money_number.sen;
        result.gosen+=ret.money_number.gosen;
        result.ichiman+=ret.money_number.ichiman;
        hon_list=ret.hon_list;
    }
    return result.hyaku;
}

pub fn test_combination_gohyaku2()->i32{
    let hon1 = Hon{
        id: 1,
        price: 110,
        hanpusu: 100,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon2 = Hon{
        id: 2,
        price: 460,
        hanpusu: 100,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon3 = Hon{
        id: 3,
        price: 740,
        hanpusu: 40,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon4 = Hon{
        id: 4,
        price: 880,
        hanpusu: 100,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let mut hon_list=Vec::new();
    hon_list.push(hon1);
    hon_list.push(hon2);
    hon_list.push(hon3);
    hon_list.push(hon4);

    let ret_money_number=MoneyNumber{
        ju: 0,
        goju: 0,
        hyaku: 0,
        gohyaku: 0,
        sen: 0,
        gosen: 0,
        ichiman: 0
    };
    let mut result=MoneyNumber{
        ju: 0,
        goju: 0,
        hyaku: 0,
        gohyaku: 0,
        sen: 0,
        gosen: 0,
        ichiman: 0
    };
    let mut ret=CalcResult{
        money_number: ret_money_number,
        hon_list: Vec::new(),
        end_flag: false
    }; 
    while !ret.end_flag {
        ret=calc_combination(hon_list);
        result.ju+=ret.money_number.ju;
        result.goju+=ret.money_number.goju;
        result.hyaku+=ret.money_number.hyaku;
        result.gohyaku+=ret.money_number.gohyaku;
        result.sen+=ret.money_number.sen;
        result.gosen+=ret.money_number.gosen;
        result.ichiman+=ret.money_number.ichiman;
        hon_list=ret.hon_list;
    }
    return result.gohyaku;
}

pub fn test_combination_sen2()->i32{
    let hon1 = Hon{
        id: 1,
        price: 110,
        hanpusu: 100,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon2 = Hon{
        id: 2,
        price: 460,
        hanpusu: 100,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon3 = Hon{
        id: 3,
        price: 740,
        hanpusu: 40,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon4 = Hon{
        id: 4,
        price: 880,
        hanpusu: 100,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let mut hon_list=Vec::new();
    hon_list.push(hon1);
    hon_list.push(hon2);
    hon_list.push(hon3);
    hon_list.push(hon4);

    let ret_money_number=MoneyNumber{
        ju: 0,
        goju: 0,
        hyaku: 0,
        gohyaku: 0,
        sen: 0,
        gosen: 0,
        ichiman: 0
    };
    let mut result=MoneyNumber{
        ju: 0,
        goju: 0,
        hyaku: 0,
        gohyaku: 0,
        sen: 0,
        gosen: 0,
        ichiman: 0
    };
    let mut ret=CalcResult{
        money_number: ret_money_number,
        hon_list: Vec::new(),
        end_flag: false
    }; 
    while !ret.end_flag {
        ret=calc_combination(hon_list);
        result.ju+=ret.money_number.ju;
        result.goju+=ret.money_number.goju;
        result.hyaku+=ret.money_number.hyaku;
        result.gohyaku+=ret.money_number.gohyaku;
        result.sen+=ret.money_number.sen;
        result.gosen+=ret.money_number.gosen;
        result.ichiman+=ret.money_number.ichiman;
        hon_list=ret.hon_list;
    }
    return result.sen;
}

pub fn test_combination_gosen2()->i32{
    // 110 100
    // 460 100
    // 740 40
    // 880 100
    // 10 : 145
    // 50 : 23
    // 100 : -7
    // 500 : -7
    // 1000 : -95
    // 5000 : 0
    let hon1 = Hon{
        id: 1,
        price: 110,
        hanpusu: 100,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon2 = Hon{
        id: 2,
        price: 460,
        hanpusu: 100,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon3 = Hon{
        id: 3,
        price: 740,
        hanpusu: 40,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon4 = Hon{
        id: 4,
        price: 880,
        hanpusu: 100,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let mut hon_list=Vec::new();
    hon_list.push(hon1);
    hon_list.push(hon2);
    hon_list.push(hon3);
    hon_list.push(hon4);

    let ret_money_number=MoneyNumber{
        ju: 0,
        goju: 0,
        hyaku: 0,
        gohyaku: 0,
        sen: 0,
        gosen: 0,
        ichiman: 0
    };
    let mut result=MoneyNumber{
        ju: 0,
        goju: 0,
        hyaku: 0,
        gohyaku: 0,
        sen: 0,
        gosen: 0,
        ichiman: 0
    };
    let mut ret=CalcResult{
        money_number: ret_money_number,
        hon_list: Vec::new(),
        end_flag: false
    }; 
    while !ret.end_flag {
        ret=calc_combination(hon_list);
        result.ju+=ret.money_number.ju;
        result.goju+=ret.money_number.goju;
        result.hyaku+=ret.money_number.hyaku;
        result.gohyaku+=ret.money_number.gohyaku;
        result.sen+=ret.money_number.sen;
        result.gosen+=ret.money_number.gosen;
        result.ichiman+=ret.money_number.ichiman;
        hon_list=ret.hon_list;
    }
    return result.gosen;
}

pub fn test_combination_ju3()->i32{
    let hon1 = Hon{
        id: 1,
        price: 1200,
        hanpusu: 2000,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon2 = Hon{
        id: 2,
        price: 500,
        hanpusu: 2000,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon3 = Hon{
        id: 3,
        price: 600,
        hanpusu: 2000,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon4 = Hon{
        id: 4,
        price: 500,
        hanpusu: 1000,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon5 = Hon{
        id: 5,
        price: 500,
        hanpusu: 1000,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon6 = Hon{
        id: 6,
        price: 400,
        hanpusu: 500,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon7 = Hon{
        id: 7,
        price: 1000,
        hanpusu: 2000,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon8 = Hon{
        id: 8,
        price: 1000,
        hanpusu: 2000,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let mut hon_list=Vec::new();
    hon_list.push(hon1);
    hon_list.push(hon2);
    hon_list.push(hon3);
    hon_list.push(hon4);
    hon_list.push(hon5);
    hon_list.push(hon6);
    hon_list.push(hon7);
    hon_list.push(hon8);

    let ret_money_number=MoneyNumber{
        ju: 0,
        goju: 0,
        hyaku: 0,
        gohyaku: 0,
        sen: 0,
        gosen: 0,
        ichiman: 0
    };
    let mut result=MoneyNumber{
        ju: 0,
        goju: 0,
        hyaku: 0,
        gohyaku: 0,
        sen: 0,
        gosen: 0,
        ichiman: 0
    };
    let mut ret=CalcResult{
        money_number: ret_money_number,
        hon_list: Vec::new(),
        end_flag: false
    }; 
    while !ret.end_flag {
        ret=calc_combination(hon_list);
        result.ju+=ret.money_number.ju;
        result.goju+=ret.money_number.goju;
        result.hyaku+=ret.money_number.hyaku;
        result.gohyaku+=ret.money_number.gohyaku;
        result.sen+=ret.money_number.sen;
        result.gosen+=ret.money_number.gosen;
        result.ichiman+=ret.money_number.ichiman;
        hon_list=ret.hon_list;
    }
    return result.ju;
}

pub fn test_combination_goju3()->i32{
    let hon1 = Hon{
        id: 1,
        price: 1200,
        hanpusu: 2000,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon2 = Hon{
        id: 2,
        price: 500,
        hanpusu: 2000,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon3 = Hon{
        id: 3,
        price: 600,
        hanpusu: 2000,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon4 = Hon{
        id: 4,
        price: 500,
        hanpusu: 1000,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon5 = Hon{
        id: 5,
        price: 500,
        hanpusu: 1000,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon6 = Hon{
        id: 6,
        price: 400,
        hanpusu: 500,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon7 = Hon{
        id: 7,
        price: 1000,
        hanpusu: 2000,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon8 = Hon{
        id: 8,
        price: 1000,
        hanpusu: 2000,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let mut hon_list=Vec::new();
    hon_list.push(hon1);
    hon_list.push(hon2);
    hon_list.push(hon3);
    hon_list.push(hon4);
    hon_list.push(hon5);
    hon_list.push(hon6);
    hon_list.push(hon7);
    hon_list.push(hon8);

    let ret_money_number=MoneyNumber{
        ju: 0,
        goju: 0,
        hyaku: 0,
        gohyaku: 0,
        sen: 0,
        gosen: 0,
        ichiman: 0
    };
    let mut result=MoneyNumber{
        ju: 0,
        goju: 0,
        hyaku: 0,
        gohyaku: 0,
        sen: 0,
        gosen: 0,
        ichiman: 0
    };
    let mut ret=CalcResult{
        money_number: ret_money_number,
        hon_list: Vec::new(),
        end_flag: false
    }; 
    while !ret.end_flag {
        ret=calc_combination(hon_list);
        result.ju+=ret.money_number.ju;
        result.goju+=ret.money_number.goju;
        result.hyaku+=ret.money_number.hyaku;
        result.gohyaku+=ret.money_number.gohyaku;
        result.sen+=ret.money_number.sen;
        result.gosen+=ret.money_number.gosen;
        result.ichiman+=ret.money_number.ichiman;
        hon_list=ret.hon_list;
    }
    return result.goju;
}

pub fn test_combination_hyaku3()->i32{
    let hon1 = Hon{
        id: 1,
        price: 1200,
        hanpusu: 2000,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon2 = Hon{
        id: 2,
        price: 500,
        hanpusu: 2000,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon3 = Hon{
        id: 3,
        price: 600,
        hanpusu: 2000,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon4 = Hon{
        id: 4,
        price: 500,
        hanpusu: 1000,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon5 = Hon{
        id: 5,
        price: 500,
        hanpusu: 1000,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon6 = Hon{
        id: 6,
        price: 400,
        hanpusu: 500,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon7 = Hon{
        id: 7,
        price: 1000,
        hanpusu: 2000,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon8 = Hon{
        id: 8,
        price: 1000,
        hanpusu: 2000,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let mut hon_list=Vec::new();
    hon_list.push(hon1);
    hon_list.push(hon2);
    hon_list.push(hon3);
    hon_list.push(hon4);
    hon_list.push(hon5);
    hon_list.push(hon6);
    hon_list.push(hon7);
    hon_list.push(hon8);

    let ret_money_number=MoneyNumber{
        ju: 0,
        goju: 0,
        hyaku: 0,
        gohyaku: 0,
        sen: 0,
        gosen: 0,
        ichiman: 0
    };
    let mut result=MoneyNumber{
        ju: 0,
        goju: 0,
        hyaku: 0,
        gohyaku: 0,
        sen: 0,
        gosen: 0,
        ichiman: 0
    };
    let mut ret=CalcResult{
        money_number: ret_money_number,
        hon_list: Vec::new(),
        end_flag: false
    }; 
    while !ret.end_flag {
        ret=calc_combination(hon_list);
        result.ju+=ret.money_number.ju;
        result.goju+=ret.money_number.goju;
        result.hyaku+=ret.money_number.hyaku;
        result.gohyaku+=ret.money_number.gohyaku;
        result.sen+=ret.money_number.sen;
        result.gosen+=ret.money_number.gosen;
        result.ichiman+=ret.money_number.ichiman;
        hon_list=ret.hon_list;
    }
    return result.hyaku;
}

pub fn test_combination_gohyaku3()->i32{
    let hon1 = Hon{
        id: 1,
        price: 1200,
        hanpusu: 2000,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon2 = Hon{
        id: 2,
        price: 500,
        hanpusu: 2000,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon3 = Hon{
        id: 3,
        price: 600,
        hanpusu: 2000,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon4 = Hon{
        id: 4,
        price: 500,
        hanpusu: 1000,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon5 = Hon{
        id: 5,
        price: 500,
        hanpusu: 1000,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon6 = Hon{
        id: 6,
        price: 400,
        hanpusu: 500,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon7 = Hon{
        id: 7,
        price: 1000,
        hanpusu: 2000,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon8 = Hon{
        id: 8,
        price: 1000,
        hanpusu: 2000,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let mut hon_list=Vec::new();
    hon_list.push(hon1);
    hon_list.push(hon2);
    hon_list.push(hon3);
    hon_list.push(hon4);
    hon_list.push(hon5);
    hon_list.push(hon6);
    hon_list.push(hon7);
    hon_list.push(hon8);
    let ret_money_number=MoneyNumber{
        ju: 0,
        goju: 0,
        hyaku: 0,
        gohyaku: 0,
        sen: 0,
        gosen: 0,
        ichiman: 0
    };
    let mut result=MoneyNumber{
        ju: 0,
        goju: 0,
        hyaku: 0,
        gohyaku: 0,
        sen: 0,
        gosen: 0,
        ichiman: 0
    };
    let mut ret=CalcResult{
        money_number: ret_money_number,
        hon_list: Vec::new(),
        end_flag: false
    }; 
    while !ret.end_flag {
        ret=calc_combination(hon_list);
        result.ju+=ret.money_number.ju;
        result.goju+=ret.money_number.goju;
        result.hyaku+=ret.money_number.hyaku;
        result.gohyaku+=ret.money_number.gohyaku;
        result.sen+=ret.money_number.sen;
        result.gosen+=ret.money_number.gosen;
        result.ichiman+=ret.money_number.ichiman;
        hon_list=ret.hon_list;
    }
    return result.gohyaku;
}

pub fn test_combination_sen3()->i32{
    let hon1 = Hon{
        id: 1,
        price: 1200,
        hanpusu: 2000,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon2 = Hon{
        id: 2,
        price: 500,
        hanpusu: 2000,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon3 = Hon{
        id: 3,
        price: 600,
        hanpusu: 2000,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon4 = Hon{
        id: 4,
        price: 500,
        hanpusu: 1000,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon5 = Hon{
        id: 5,
        price: 500,
        hanpusu: 1000,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon6 = Hon{
        id: 6,
        price: 400,
        hanpusu: 500,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon7 = Hon{
        id: 7,
        price: 1000,
        hanpusu: 2000,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon8 = Hon{
        id: 8,
        price: 1000,
        hanpusu: 2000,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let mut hon_list=Vec::new();
    hon_list.push(hon1);
    hon_list.push(hon2);
    hon_list.push(hon3);
    hon_list.push(hon4);
    hon_list.push(hon5);
    hon_list.push(hon6);
    hon_list.push(hon7);
    hon_list.push(hon8);
    let ret_money_number=MoneyNumber{
        ju: 0,
        goju: 0,
        hyaku: 0,
        gohyaku: 0,
        sen: 0,
        gosen: 0,
        ichiman: 0
    };
    let mut result=MoneyNumber{
        ju: 0,
        goju: 0,
        hyaku: 0,
        gohyaku: 0,
        sen: 0,
        gosen: 0,
        ichiman: 0
    };
    let mut ret=CalcResult{
        money_number: ret_money_number,
        hon_list: Vec::new(),
        end_flag: false
    }; 
    while !ret.end_flag {
        ret=calc_combination(hon_list);
        result.ju+=ret.money_number.ju;
        result.goju+=ret.money_number.goju;
        result.hyaku+=ret.money_number.hyaku;
        result.gohyaku+=ret.money_number.gohyaku;
        result.sen+=ret.money_number.sen;
        result.gosen+=ret.money_number.gosen;
        result.ichiman+=ret.money_number.ichiman;
        hon_list=ret.hon_list;
    }
    return result.sen;
}

pub fn test_combination_gosen3()->i32{
    let hon1 = Hon{
        id: 1,
        price: 1200,
        hanpusu: 2000,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon2 = Hon{
        id: 2,
        price: 500,
        hanpusu: 2000,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon3 = Hon{
        id: 3,
        price: 600,
        hanpusu: 2000,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon4 = Hon{
        id: 4,
        price: 500,
        hanpusu: 1000,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon5 = Hon{
        id: 5,
        price: 500,
        hanpusu: 1000,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon6 = Hon{
        id: 6,
        price: 400,
        hanpusu: 500,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon7 = Hon{
        id: 7,
        price: 1000,
        hanpusu: 2000,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let hon8 = Hon{
        id: 8,
        price: 1000,
        hanpusu: 2000,
        hanpu_count: 0,
        hanpu_number_combination: 0,
        amari: 0
    };
    let mut hon_list=Vec::new();
    hon_list.push(hon1);
    hon_list.push(hon2);
    hon_list.push(hon3);
    hon_list.push(hon4);
    hon_list.push(hon5);
    hon_list.push(hon6);
    hon_list.push(hon7);
    hon_list.push(hon8);

    let ret_money_number=MoneyNumber{
        ju: 0,
        goju: 0,
        hyaku: 0,
        gohyaku: 0,
        sen: 0,
        gosen: 0,
        ichiman: 0
    };
    let mut result=MoneyNumber{
        ju: 0,
        goju: 0,
        hyaku: 0,
        gohyaku: 0,
        sen: 0,
        gosen: 0,
        ichiman: 0
    };
    let mut ret=CalcResult{
        money_number: ret_money_number,
        hon_list: Vec::new(),
        end_flag: false
    }; 
    while !ret.end_flag {
        ret=calc_combination(hon_list);
        result.ju+=ret.money_number.ju;
        result.goju+=ret.money_number.goju;
        result.hyaku+=ret.money_number.hyaku;
        result.gohyaku+=ret.money_number.gohyaku;
        result.sen+=ret.money_number.sen;
        result.gosen+=ret.money_number.gosen;
        result.ichiman+=ret.money_number.ichiman;
        hon_list=ret.hon_list;
    }
    return result.gosen;
}

fn calc_combination(mut hon_list: Vec<Hon>)->CalcResult{
    let mut ret_money_number=MoneyNumber{
        ju: 0,
        goju: 0,
        hyaku: 0,
        gohyaku: 0,
        sen: 0,
        gosen: 0,
        ichiman: 0
    };
    // 本の種類数。
    let hon_su: i32=hon_list.len().try_into().unwrap();
    let mut combination_list: Vec<HonForCalc>=Vec::new();
    let mut _index: usize=0;

    // 部分集合をビット列で取ります。
    // 一冊だけならループ数を増やさないのでループ数を2から始める。
    let mut loop_num: i32=2;
    let mut counter: i32=0;
    while counter<hon_su-1 {
        loop_num*=2;
        counter+=1;
    }
    // 本それぞれの頒布回数をカウントします。
    counter=0;
    _index=0;
    while counter<loop_num {
        let mut num: i32=counter;
        _index=0;
        while num!=0 {
            if (num & 0x0000000000000000000000000000001)==0x00000000000000000000000000000001 {
                hon_list[_index].hanpu_count+=1;
            }
            _index+=1;
            num=num>>1;
        }
        counter+=1;
    }
    // 頒布回数から組み合わせ後の頒布数を計算します。
    for hon in &mut hon_list {
        let hanpusu=hon.hanpusu;
        let hanpu_count=hon.hanpu_count;
        // 入力された本の頒布数を、組み合わせでカウントした頒布回数で割って、
        // 今回の処理の頒布数カウントを計算する。
        let hanpusu_count=hanpusu/hanpu_count;
        let amari=hanpusu%hanpu_count;
        hon.hanpu_number_combination=hanpusu_count;
        hon.amari=amari;
    }
    // 最小頒布数を見つけます。
    let mut min_hanpusu: i32=i32::MAX;
    for _hon in hon_list.iter() {
        if _hon.hanpusu<min_hanpusu && 0<_hon.hanpusu {
            min_hanpusu=_hon.hanpusu;
        }
    }
    // 頒布数が組み合わせの数を下回っていたら組み合わせを作らない。
    if loop_num<min_hanpusu {
        // 頒布数計算。
        counter=0;
        while counter<loop_num {
            let mut num: i32=counter;
            let mut shurui_su: i32=0;
            let mut sum: i32=0;
            let mut hanpusu: i32=0;
            let mut amari: i32=0;
            let mut moto_list: Vec<i32>=Vec::new();
            _index=0;
            while num!=0 {
                if (num & 0x00000000000000000000000000000001)==0x00000000000000000000000000000001 {
                    // 頒布数が0なら組み合わせに入れない。
                    if 0<hon_list[_index].hanpusu {
                        // 組み合わせで計算し、例えば200円5冊300円5冊の組み合わせなら、
                        // 500円10冊と計算する。
                        moto_list.push(hon_list[_index].id);
                        sum+=hon_list[_index].price;
                        hanpusu+=hon_list[_index].hanpu_number_combination;
                        amari+=hon_list[_index].amari;
                        shurui_su+=1;    
                    }
                }
                _index+=1;
                num=num>>1;
            }
            //println!("sum : {}",sum);
            //println!("hanpusu : {}",hanpusu);
            if sum!=0 {
                // 頒布数を種類数で割る。余ったら次のループに持ち越すので、余りは無視する。
                if shurui_su < hon_list.len().try_into().unwrap() {
                    if 0<hanpusu {
                        let hon_for_calc=HonForCalc {
                            price: sum,
                            hanpusu: hanpusu/shurui_su,
                            moto_list: moto_list
                        };
                        combination_list.push(hon_for_calc);                        
                    }
                }else{
                    // 種類数が本の種類を上回っている場合、余った冊数を寄せて計算する。
                    if 0<hanpusu {
                        let hon_for_calc=HonForCalc {
                            price: sum,
                            hanpusu: (hanpusu+amari)/shurui_su,
                            moto_list: moto_list
                        };
                        combination_list.push(hon_for_calc);
                    }
                }
            }
            counter+=1;
        }
    }else{
        // 最小頒布数が組み合わせの数に足りなかったら、組み合わせず単冊に分散させる。
        for _hon in hon_list.iter() {
            let mut tansatsu_moto_list=Vec::new();
            tansatsu_moto_list.push(_hon.id);
            let hon_for_calc=HonForCalc {
                price: _hon.price,
                hanpusu: _hon.hanpusu,
                moto_list: tansatsu_moto_list
            };
            combination_list.push(hon_for_calc);
        }
    }
    // 最小頒布数を見つける。
    let mut hanpusu_for_calc: i32=i32::MAX;
    for hon in combination_list.iter() {
        if hon.hanpusu<hanpusu_for_calc && 0<hon.hanpusu {
            hanpusu_for_calc=hon.hanpusu;
        }
    }
    // 最小頒布数が見つからず全て0以下なら終了して帰る。
    if hanpusu_for_calc==i32::MAX {
        let ret=CalcResult{
            money_number: ret_money_number,
            hon_list: hon_list,
            end_flag: true
        };    
        return ret;
    }
    // お釣りの枚数を数えます。
    for hon in combination_list.iter() {
        if 0<hon.hanpusu {
            let ret_calc=calc_ikura(hon.price,hanpusu_for_calc);
            ret_money_number.ju+=ret_calc.ju;
            ret_money_number.goju+=ret_calc.goju;
            ret_money_number.hyaku+=ret_calc.hyaku;
            ret_money_number.gohyaku+=ret_calc.gohyaku;
            ret_money_number.sen+=ret_calc.sen;
            ret_money_number.gosen+=ret_calc.gosen;
            ret_money_number.ichiman+=ret_calc.ichiman;
        }
    }
    // 元々のIDを参照して頒布数を引きます。
    for _hon in &mut hon_list {
        for hon_for_calc in combination_list.iter() {
            let moto_list=&hon_for_calc.moto_list;
            for hon_moto in moto_list.iter() {
                if _hon.id==*hon_moto {
                    // 計算した分の頒布数を引いて次の処理に持ち越す。
                    if 0<_hon.hanpusu {
                        _hon.hanpusu-=hanpusu_for_calc;
                    }
                    // 本の頒布回数をクリアーする。
                    _hon.hanpu_count=0;
                }
            }
        }
    }
    let ret=CalcResult{
        money_number: ret_money_number,
        hon_list: hon_list,
        end_flag: false
    };
    return ret;
}

fn calc_ikura(price: i32, hanpusu: i32)->MoneyNumber{
    let mut ret=MoneyNumber{
        ju: 0,
        goju: 0,
        hyaku: 0,
        gohyaku: 0,
        sen: 0,
        gosen: 0,
        ichiman: 0
    };

    // 頒布数が0以下なら帰る。
    if hanpusu <= 0 {
        return ret;
    }

    if price < 100 {
        ret=calc_ju(price,hanpusu);
    } else if 100 <= price && price < 1000 {
        ret=calc_hyaku(price,hanpusu);
    } else if 1000 <= price && price < 3000 {
        // 1000円から3000円までの場合、1万円の使用は想定せず、1000円台をなくして考える。
        let sen_price=price%1000;
        if sen_price < 100 {
            ret=calc_ju(sen_price,hanpusu);
        } else if 100<= sen_price && sen_price < 1000 {
            ret=calc_hyaku(sen_price,hanpusu);
        }
    } else if 3000 <= price && price < 10000 {
        ret=calc_sen(price,hanpusu);
    } else if 10000 <= price {
        ret=calc_man(price,hanpusu);
    }
    return ret;
}

fn calc_ju(price: i32, hanpusu: i32)->MoneyNumber{
    let mut price_list: Vec<i32>=Vec::new();
    if price < 50 {
        price_list.push(price);
        price_list.push(50);
        price_list.push(100);
    } else if price == 50 {
        price_list.push(price);
        price_list.push(100);
    } else if 50 < price {
        price_list.push(price);
        price_list.push(100);
    }
    return calc_oturi_money_number(price_list, hanpusu);
}

fn calc_hyaku(_price: i32, hanpusu: i32)->MoneyNumber{
    let mut price_list: Vec<i32>=Vec::new();
    let mut price: i32 = 0;
    // 1000円から3000円までも本メソッドで計算して、
    // 1000円から3000円までの場合1万円の使用を想定しない。
    if price <= 1000 {
        price=_price%1000;
    }else{
        price=_price;
    }

    let price_judai: i32=price%100;
    let price_hyakudai: i32=(price-price_judai)%1000;
    if price < 500 {
        if price_judai == 0 {
            price_list.push(price);
            price_list.push(500);
            price_list.push(1000);
        }else if price_hyakudai != 400 {
            price_list.push(price);
            price_list.push(price+(100-price_judai));
            price_list.push(500);
            price_list.push(1000);
        }else if price_hyakudai == 400 {
            price_list.push(price);
            price_list.push(500);
            price_list.push(1000);
        }
    }else if price == 500 {
        price_list.push(price);
        price_list.push(1000);
    }else if 500 < price && price < 1000 {
        price_list.push(price);
        price_list.push(1000);
    }
    return calc_oturi_money_number(price_list, hanpusu);
}

fn calc_sen(price: i32, hanpusu: i32)->MoneyNumber{
    let mut price_list: Vec<i32>=Vec::new();
    if 3000 <= price && price < 4000 {
        price_list=calc_sen_under_yonsen(price);
    }else if 4000 <= price && price < 5000 {
        price_list=calc_sen_yonsen(price);
    }else if 5000 == price {
        price_list=calc_sen_return_list_daisatu(5000,10000);
    }else if 5000 < price && price < 9000 {
        price_list=calc_sen_under_kyusen(price);
    }else if 9000 <= price {
        price_list=calc_sen_kyusen(price);
    }
    return calc_oturi_money_number(price_list, hanpusu);
}

fn calc_man(price: i32, hanpusu: i32)->MoneyNumber{
    // 2万円より上の場合、1万円の枚数を引いて計算して、手元に残る枚数に足します。

    // 12000,15000,20000
    // 12500,13000,15000,20000
    // 15000,20000
    // 15500,16000,20000
    // 16500,17000,20000
    // 18000,20000
    // 18500,19000,20000
    // 19000,20000
    // 19500,20000
    let mut _price_list: Vec<i32>=Vec::new();
    // TODO : 揃えます。
    let ichiman_money_number=calc_ichiman_money_number(price);
    let gosen_money_number=calc_gosen_money_number(price);
    let sen_money_number=calc_sen_money_number(price);
    let hyaku_money_number=calc_hyaku_money_number(price);
    let gohyaku_money_number=calc_gohyaku_money_number(price);
    // 20000円より上の場合、10000円台に落として計算する。
    let price_for_calc=10000+gosen_money_number*5000+sen_money_number*1000+gohyaku_money_number*500+hyaku_money_number*100;
    if hyaku_money_number==0 && gohyaku_money_number==0 {
        _price_list=calc_man_sen(price_for_calc);
    }else{
        _price_list=calc_man_hyaku(price_for_calc);
    }
    let mut ret=calc_oturi_money_number(_price_list,hanpusu);
    ret.ichiman-=ichiman_money_number-1;
    return ret;
}

fn calc_man_hyaku(price: i32)->Vec<i32>{
    // 12300,12500,13000,15000
    // 12500,13000,15000
    // 14500,15000
    // 15500,16000,20000
    // 16500,17000,20000
    // 18500,19000,20000
    // 19500,20000
    let mut ret: Vec<i32>=Vec::new();
    let price_hyakudai: i32=price%1000;
    let price_sendai: i32=price-10000-price_hyakudai;
    ret.push(price);
    if price_hyakudai<500 {
        ret.push(10000+price_sendai+500);
    }
    ret.push(10000+price_sendai+1000);
    if price_sendai<5000 && price_sendai+1000!=5000 {
        ret.push(15000);
    }
    if 5000<=price_sendai && price_sendai+1000!=10000 {
        ret.push(20000);
    }
    return ret;
}

fn calc_man_sen(price: i32)->Vec<i32>{
    // 12000,15000
    // 15000,20000
    // 18000,20000
    // 19000,20000
    // 19500,20000
    let mut ret: Vec<i32>=Vec::new();
    let price_hyakudai: i32=price%1000;
    let price_sendai: i32=price-10000-price_hyakudai;
    ret.push(price);
    if price_sendai<5000 {
        ret.push(15000);
    }
    if 5000<=price_sendai {
        ret.push(20000);
    }
    return ret;
}

fn calc_sen_under_yonsen(price: i32)->Vec<i32>{
    // TODO : リファクタリングする。
    let price_judai: i32=price%100;
    let price_hyakudai: i32=(price-price_judai)%1000;
    let price_sendai: i32=price-price_hyakudai-price_judai;
    let mut ret: Vec<i32>=Vec::new();
    if price_hyakudai==0 {
        if price_judai==0 {
            // 3000,5000
            ret=calc_sen_return_list_daisatu(price,5000);
        }else if 0<price_judai && price_judai<50{
            // 3010,3050,3100,3500,4000,5000
            ret=calc_sen_return_list_goju(price,price_hyakudai,price_sendai,5000);
        }else if 50<=price_judai {
            // 3050,3100,3500,4000,5000
            // 3060,3100,3500,4000,5000
            // 3090,3100,3500,4000,5000
            ret=calc_sen_return_list_hyaku(price,price_hyakudai,price_sendai,5000);
        }
    }else if 0<price_hyakudai && price_hyakudai<400 {
        if price_judai==0 {
            // 3100,3500,4000,5000
            ret=calc_sen_return_list_gohyaku(price,price_sendai,5000);
        }else if 0<price_judai && price_judai<50 {
            // 3110,3150,3200,3500,4000,5000
            ret=calc_sen_return_list_goju(price,price_hyakudai,price_sendai,5000);
        }else if 50<=price_judai {
            // 3150,3200,3500,4000,5000
            // 3160,3200,3500,4000,5000
            // 3190,3200,3500,4000,5000
            ret=calc_sen_return_list_hyaku(price,price_hyakudai,price_sendai,5000);
        }
    }else if 400<=price_hyakudai && price_hyakudai<500 {
        if price_judai==0 {
            // 3400,3500,4000,5000
            ret=calc_sen_return_list_gohyaku(price,price_sendai,5000);
        }else if 0<price_judai && price_judai<50 {
            // 3410,3450,3500,4000,5000
            ret=calc_sen_return_list_goju(price,price_hyakudai,price_sendai,5000);
        }else if 50<=price_judai {
            // 3450,3500,4000,5000
            // 3460,3500,4000,5000
            ret=calc_sen_return_list_gohyaku(price,price_sendai,5000);
        }
    }else if price_hyakudai==500 {
        if price_judai==0 {
            // 3500,4000,5000
            ret=calc_sen_return_list_sen(price,price_sendai,5000);
        }else if 0<price_judai && price_judai<50 {
            // 3510,3550,3600,4000,5000
            ret=calc_sen_return_list_goju(price,price_hyakudai,price_sendai,5000);
        }else if 50<=price_judai {
            // 3550,3600,4000,5000
            // 3560,3600,4000,5000
            ret=calc_sen_return_list_hyaku(price,price_hyakudai,price_sendai,5000);
        }
    }else if 500<price_hyakudai && price_hyakudai<900 {
        if price_judai==0 {
            // 3600,4000,5000
            ret=calc_sen_return_list_sen(price,price_sendai,5000);
        }else if 0<price_judai && price_judai<50 {
            // 3610,3650,3700,4000,5000
            ret=calc_sen_return_list_goju(price,price_hyakudai,price_sendai,5000);
        }else if 50<=price_judai {
            // 3660,3700,4000,5000
            ret=calc_sen_return_list_hyaku(price,price_hyakudai,price_sendai,5000);            
        }
    }else if 900<=price_sendai {
        if price_judai==0 {
            // 3900,4000,5000
            ret=calc_sen_return_list_sen(price,price_sendai,5000);
        }else if 0<price_judai && price_judai<50 {
            // 3910,3950,4000,5000
            ret=calc_sen_return_list_goju(price,price_hyakudai,price_sendai,5000);
        }else if 50<=price_judai {
            // 3950,4000,5000
            // 3960,4000,5000 
            ret=calc_sen_return_list_sen(price,price_sendai,5000);
        }
    }
    return ret;
}

fn calc_sen_yonsen(price: i32)->Vec<i32>{
    // TODO : 揃える。
    let price_judai: i32=price%100;
    let price_hyakudai: i32=(price-price_judai)%1000;
    let price_sendai: i32=price-price_hyakudai-price_judai;
    let mut ret: Vec<i32>=Vec::new();
    if price_hyakudai==0 {
        if price_judai==0{
            // 4000,5000
            ret=calc_sen_return_list_daisatu(price,5000);        
        }else if 0<price_judai && price_judai<50 {
            // 4010,4050,4100,4500,5000
            ret=calc_sen_return_list_goju(price,price_hyakudai,price_sendai,5000);
        }else if 50<=price_judai {
            // 4050,4100,4500,5000
            // 4060,4100,4500,5000
            ret=calc_sen_return_list_hyaku(price,price_hyakudai,price_sendai,5000);
        }
    }else if 0<price_hyakudai && price_hyakudai<400 {
        if price_judai==0 {
            // 4100,4500,5000
            ret=calc_sen_return_list_gohyaku(price,price_sendai,5000);
        }else if 0<price_judai && price_judai<50 {
            // 4110,4150,4200,4500,5000
            ret=calc_sen_return_list_goju(price,price_hyakudai,price_sendai,5000);
        }else if 50<=price_judai {
            // 4150,4200,4500,5000
            // 4160,4200,4500,5000
            ret=calc_sen_return_list_hyaku(price,price_hyakudai,price_sendai,5000);
        }
    }else if 400<=price_hyakudai && price_hyakudai<500 {
        if price_judai==0 {
            // 4400,4500,5000
            ret=calc_sen_return_list_gohyaku(price,price_sendai,5000);
        }else if 0<price_judai && price_judai<50 {
            // 4410,4450,4500,5000
            ret=calc_sen_return_list_goju(price,price_hyakudai,price_sendai,5000);
        }else if 50<=price_judai {
            // 4450,4500,5000
            // 4460,4500,5000
            ret=calc_sen_return_list_gohyaku(price,price_sendai,5000);
        }
    }else if 500<=price_hyakudai && price_hyakudai<900 {
        if price_judai==0 {
            // 4500,5000
            // 4600,5000
            ret=calc_sen_return_list_daisatu(price,5000);
        }else if 0<price_judai && price_judai<50 {
            // 4510,4550,4600,5000
            // 4610,4650,4700,5000
            ret=calc_sen_return_list_goju(price,price_hyakudai,price_sendai,5000);
        }else if 50<=price_judai {
            // 4550,4600,5000
            // 4560,4600,5000
            // 4650,4700,5000
            // 4660,4700,5000
            ret=calc_sen_return_list_hyaku(price,price_hyakudai,price_sendai,5000);
        }
    }else if 900<=price_hyakudai {
        if price_judai==0 {
            // 4900,5000
            ret=calc_sen_return_list_daisatu(price,5000);
        }else if 0<price_judai && price_judai<50 {
            // 4910,4950,5000
            ret=calc_sen_return_list_goju(price,price_hyakudai,price_sendai,5000);
        }else if 50<=price_judai {
            // 4950,5000
            // 4960,5000
            ret=calc_sen_return_list_daisatu(price,5000);
        }
    }
    return ret;
}

fn calc_sen_under_kyusen(price: i32)->Vec<i32>{
    let price_judai: i32=price%100;
    let price_hyakudai: i32=(price-price_judai)%1000;
    let price_sendai: i32=price-price_hyakudai-price_judai;
    let mut ret: Vec<i32>=Vec::new();

    if price_hyakudai==0 {
        if price_judai==0 {
            // 6000,10000
            ret=calc_sen_return_list_daisatu(price,10000);
        }else if 0<price_judai && price_judai<50 {
            // 5010,5050,5100,5500,6000,10000
            ret=calc_sen_return_list_goju(price,price_hyakudai,price_sendai,10000);
        }else if 50<=price_judai {
            // 5050,5100,5500,6000,10000
            // 5060,5100,5500,6000,10000
            ret=calc_sen_return_list_hyaku(price,price_hyakudai,price_sendai,10000);
        }
    }else if 0<price_hyakudai && price_hyakudai<400 {
        if price_judai==0 {
            // 5100,5500,6000,10000
            ret=calc_sen_return_list_gohyaku(price,price_sendai,10000);
        }else if 0<price_judai && price_judai<50 {
            // 5110,5150,5200,5500,6000,10000
            ret=calc_sen_return_list_goju(price,price_hyakudai,price_sendai,10000);
        }else if 50<=price_judai {
            // 5150,5200,5500,6000,10000
            // 5160,5200,5500,6000,10000
            ret=calc_sen_return_list_hyaku(price,price_hyakudai,price_sendai,10000);
        }
    }else if 400<=price_hyakudai && price_hyakudai<500 {
        if price_judai==0 {
            // 5400,5500,6000,10000
            ret=calc_sen_return_list_gohyaku(price,price_sendai,10000);
        }else if 0<price_judai && price_judai<50 {
            // 5410,5450,5500,6000,10000
            ret=calc_sen_return_list_goju(price,price_hyakudai,price_sendai,10000);
        }else if price_judai==50 {
            // 5450,5500,6000,10000
            ret=calc_sen_return_list_gohyaku(price,price_sendai,10000);
        }else if 50<=price_judai {
            // 5460,5500,6000,10000
            // 5490,5500,6000,10000
            ret=calc_sen_return_list_gohyaku(price,price_sendai,10000);
        }
    }else if 500<=price_hyakudai && price_hyakudai<900 {
        if price_judai==0 {
            // 5500,6000,10000
            // 5600,6000,10000
            ret=calc_sen_return_list_sen(price,price_sendai,10000);
        }else if 0<price_judai && price_judai<50 {
            // 5510,5550,5600,6000,10000
            // 5610,5650,5700,6000,10000
            ret=calc_sen_return_list_goju(price,price_hyakudai,price_sendai,10000);
        }else if 50<=price_judai {
            // 5560,5600,6000,10000
            // 5590,5600,6000,10000
            // 5660,5700,6000,10000
            // 5690,5700,6000,10000
            ret=calc_sen_return_list_hyaku(price,price_hyakudai,price_sendai,10000);
        }
    }else if 900<=price_hyakudai {
        if price_judai==0 {
            // 5900,6000,10000
            ret=calc_sen_return_list_sen(price,price_sendai,10000);
        }else if 0<price_judai && price_judai<50 {
            // 5910,5950,6000,10000
            ret=calc_sen_return_list_goju(price,price_hyakudai,price_sendai,10000);
        }else if 50<=price_judai {
            // 5950,6000,10000
            // 5960,6000,10000
            // 5990,6000,10000
            ret=calc_sen_return_list_sen(price,price_sendai,10000);
        }
    }
    return ret;
}

fn calc_sen_kyusen(price: i32)->Vec<i32>{
    let price_judai: i32=price%100;
    let price_hyakudai: i32=(price-price_judai)%1000;
    let price_sendai: i32=price-price_hyakudai-price_judai;
    let mut ret: Vec<i32>=Vec::new();

    if price_hyakudai==0 {
        if price_judai==0 {
            // 9000,10000
            ret=calc_sen_return_list_daisatu(price,10000);
        }else if 0<price_judai && price_judai<50 {
            // 9010,9050,9100,9500,10000
            ret=calc_sen_return_list_goju(price,price_hyakudai,price_sendai,10000);
        }else if 50<=price_judai {
            // 9050,9100,9500,10000
            // 9060,9100,9500,10000
            ret=calc_sen_return_list_hyaku(price,price_hyakudai,price_sendai,10000);
        }
    }else if 0<price_hyakudai && price_hyakudai<500 {
        if price_judai==0 {
            // 9100,9500,10000
            ret=calc_sen_return_list_gohyaku(price,price_sendai,10000);
        }else if 0<price_judai && price_judai<50 {
            // 9110,9150,9200,9500,10000
            ret=calc_sen_return_list_goju(price,price_hyakudai,price_sendai,10000);
        }else if 50<=price_judai {
            // 9150,9200,9500,10000
            // 9160,9200,9500,10000
            ret=calc_sen_return_list_hyaku(price,price_hyakudai,price_sendai,10000);
        }
    }else if price_hyakudai==500 {
        if price_judai==0 {
            // 9500,10000
            ret=calc_sen_return_list_daisatu(price,10000);
        }else if 0<price_judai && price_judai<50 {
            // 9510,9550,9600,10000
            ret=calc_sen_return_list_goju(price,price_hyakudai,price_sendai,10000);
        }else if 50<=price_judai {
            // 9550,9600,10000
            // 9560,9600,10000
            ret=calc_sen_return_list_hyaku(price,price_hyakudai,price_sendai,10000);
        }
    }else if 500<price_hyakudai && price_hyakudai<900 {
        if price_judai==0 {
            // 9600,10000
            ret=calc_sen_return_list_daisatu(price,10000);
        }else if 0<price_judai && price_judai<50 {
            // 9610,9650,9700,10000
            ret=calc_sen_return_list_goju(price,price_hyakudai,price_sendai,10000);
        }else if 50<=price_judai {
            // 9650,9700,10000
            // 9660,9700,10000
            ret=calc_sen_return_list_hyaku(price,price_hyakudai,price_sendai,10000);
        }
    }else if 900<=price_hyakudai {
        if price_judai==0 {
            // 9900,10000
            ret=calc_sen_return_list_daisatu(price,10000);
        }else if 0<price_judai && price_judai<50 {
            // 9910,9950,10000
            ret=calc_sen_return_list_goju(price,price_hyakudai,price_sendai,10000);
        }else if 50<=price_judai {
            // 9950,10000
            // 9960,10000
            ret=calc_sen_return_list_daisatu(price,10000);
        }
    }
    return ret;
}

fn calc_sen_return_list_goju(price:i32, price_hyakudai: i32, price_sendai: i32, daisatsu: i32)->Vec<i32>{
    let mut ret: Vec<i32>=Vec::new();
    let next_goju=(price_sendai+price_hyakudai+50)-price;
    ret.push(price);
    ret.push(price+next_goju);
    if price_hyakudai+100 != 500 || price_hyakudai+100 != 1000 {
        ret.push(price_sendai+price_hyakudai+100);
    }
    if price_hyakudai<500 && price_hyakudai+100!=500{
        ret.push(price_sendai+500);
    }
    if price_hyakudai+100!=1000{
        ret.push(price_sendai+1000);
    }
    if price_sendai+1000!=daisatsu {
        ret.push(daisatsu);
    }
    return ret;
}

fn calc_sen_return_list_hyaku(price: i32, price_hyakudai: i32, price_sendai: i32, daisatsu: i32)->Vec<i32>{
    let mut ret: Vec<i32>=Vec::new();
    let next_hyaku=price_sendai+price_hyakudai+100;
    ret.push(price);
    ret.push(next_hyaku);
    if price_hyakudai<500 && price_hyakudai+100!=500 {
       ret.push(price_sendai+500);
    }
    ret.push(price_sendai+1000);
    if price_sendai+1000!=daisatsu {
        ret.push(daisatsu);
    }
    return ret;
}

fn calc_sen_return_list_gohyaku(price: i32, price_sendai: i32, daisatsu: i32)->Vec<i32>{
    let mut ret: Vec<i32>=Vec::new();
    ret.push(price);
    ret.push(price_sendai+500);
    ret.push(price_sendai+1000);
    if price_sendai+1000!=daisatsu {
        ret.push(daisatsu);
    }
    return ret;
}

fn calc_sen_return_list_sen(price: i32, price_sendai: i32, daisatsu: i32)->Vec<i32>{
    let mut ret: Vec<i32>=Vec::new();
    ret.push(price);
    ret.push(price_sendai+1000);
    if price_sendai+1000!=daisatsu {
        ret.push(daisatsu);
    }    
    return ret;
}

fn calc_sen_return_list_daisatu(price: i32, daisatsu: i32)->Vec<i32>{
    let mut ret: Vec<i32>=Vec::new();
    ret.push(price);
    ret.push(daisatsu);
    return ret;
}

fn calc_oturi_money_number(price_list: Vec<i32>, hanpusu: i32)->MoneyNumber{

    let mut ninzu_list: Vec<i32>=Vec::new();
    let price_shurui: i32=price_list.len() as i32;
    let ninzu: i32=hanpusu/price_shurui;
    let price: i32=price_list[0];
    for _price in price_list.iter(){
        ninzu_list.push(ninzu);
    }
    // 余りが出るなら、大きい札で辻褄を合わせる。
    if hanpusu%price_shurui != 0 {
        let mut amari: i32=0;
        for _ninzu in ninzu_list.iter(){
            amari+=_ninzu;
        }
        let ninzu_size: usize=ninzu_list.len()-1;
        ninzu_list[ninzu_size]+=hanpusu-amari;
    }
    // 頒布数が幾らの人数に足りない場合、0にする。
    for ninzu in ninzu_list.iter_mut(){
        if *ninzu < 0 {
            *ninzu=0;
        }
    }

    let mut harau_list: Vec<Pay>=Vec::new();
    for i in 0..price_shurui {
        let mut pay=Pay {
            price: 0,
            ninzu: 0        
        };
        let index: usize=i as usize;
        pay.price=price_list[index];
        pay.ninzu=ninzu_list[index];
        harau_list.push(pay);
    }
    return count_money_number(price,harau_list);
}

fn count_money_number(price: i32,harau_list: Vec<Pay>)->MoneyNumber{
    let mut ret=MoneyNumber{
        ju: 0,
        goju: 0,
        hyaku: 0,
        gohyaku: 0,
        sen: 0,
        gosen: 0,
        ichiman: 0
    };
    // お釣りで払う枚数を計算します。
    for pay in harau_list.iter(){
        if price<pay.price {
            let oturi=pay.price-price;
            ret.ju+=calc_ju_money_number(oturi)*pay.ninzu;
            ret.goju+=calc_goju_money_number(oturi)*pay.ninzu;
            ret.hyaku+=calc_hyaku_money_number(oturi)*pay.ninzu;
            ret.gohyaku+=calc_gohyaku_money_number(oturi)*pay.ninzu;
            ret.sen+=calc_sen_money_number(oturi)*pay.ninzu;
            ret.gosen+=calc_gosen_money_number(oturi)*pay.ninzu;
        }
    }

    // 入ってくる枚数を計算します。
    for pay in harau_list.iter(){
        ret.ju-=calc_ju_money_number(pay.price)*pay.ninzu;
        ret.goju-=calc_goju_money_number(pay.price)*pay.ninzu;
        ret.hyaku-=calc_hyaku_money_number(pay.price)*pay.ninzu;
        ret.gohyaku-=calc_gohyaku_money_number(pay.price)*pay.ninzu;
        ret.sen-=calc_sen_money_number(pay.price)*pay.ninzu;
        ret.gosen-=calc_gosen_money_number(pay.price)*pay.ninzu;
        ret.ichiman-=calc_ichiman_money_number(pay.price)*pay.ninzu;
    }
    return ret;
}

fn calc_ichiman_money_number(money: i32)->i32{    
    let mut ret: i32=0;
    if 5000<=money {
        ret=(money/10000)%10;
    }
    return ret;
}

fn calc_gosen_money_number(money: i32)->i32{
    let mut ret: i32=0;
    if 5000<=money {
        let gosen=(money/1000)%10;
        if 5<=gosen {
            ret=1;
        }
    }
    return ret;
}

fn calc_sen_money_number(money: i32)->i32{
    let mut ret: i32=0;
    if 1000<= money{
        let sen=(money/1000)%10;
        if 0<sen && sen<5 {
            ret=sen;
        }else if sen==5 {
            ret=0;
        }else if 5<sen && sen<=9 {
            ret=sen-5;
        }
    }
    return ret;
}

fn calc_gohyaku_money_number(money: i32)->i32{
    let mut ret: i32=0;
    if 500<=money {
        let gohyaku: i32=(money/100)%10;
        if 5<=gohyaku {
            ret=1;
        }
    }
    return ret;
}

fn calc_hyaku_money_number(money: i32)->i32{
    let mut ret: i32=0;
    if 100<=money {
        let hyaku: i32=(money/100)%10;
        if 0<hyaku && hyaku<5 {
            ret=hyaku;
        }else if hyaku==5 {
            ret=0;
        }else if 5<hyaku && hyaku<=9 {
            ret=hyaku-5;
        }
    }
    return ret;
}

fn calc_goju_money_number(money: i32)->i32{
    let mut ret: i32=0;
    if 50<=money {
        let goju: i32=(money/10)%10;
        if 5<=goju {
            ret=1;
        }
    }
    return ret;
}

fn calc_ju_money_number(money: i32)->i32{
    let mut ret: i32=0;
    if 10<=money {
        let ju: i32=(money/10)%10;
        if 0<ju && ju<5 {
            ret=ju;
        }else if ju==5 {
            ret=0;
        }else if 5<ju && ju<=9 {
            ret=ju-5;
        }
    }
    return ret;
}