pub fn pb1() {
    let total: i64 = INPUT.lines().map(to_dec).sum();
    assert_eq!(&to_snafu(total), "20==1==12=0111=2--20");
}

fn to_snafu(mut input: i64) -> String {
    let mut snafu: Vec<char> = vec![];
    while input != 0 {
        let rem: i64 = input % 5;
        let digit = match rem {
            3 => '=',
            4 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => panic!(),
        };
        snafu.push(digit);
        input = (input + 2) / 5;
    }
    snafu.iter().rev().collect::<String>()
}

fn to_dec(input: &str) -> i64 {
    input.chars().fold(0, |acc, c| {
        acc * 5
            + match c {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '-' => -1,
                '=' => -2,
                _ => panic!(),
            }
    })
}
#[allow(dead_code)]
const INPUT_CUSTOM: &str = "";

#[allow(dead_code)]
const INPUT_TEST: &str = "\
1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

#[allow(dead_code)]
const INPUT: &str = "\
1--
1-
1--02
1122-=2-==0-0=1-2
2=1-==-1022=10
1==-02211022=000
101==210-21=2
11010==0==221=0=02
11=1101020=02-11
1===
10-2
10=02=-=02
1-1--112-202--0
11-=-0-002=0112-1
2--0-2=02=-22
1001-2
1=101=2=2102-201--1
1-2==1100--
2-01=--=1-11
101-=0010-210==
1=-21-022121110
1==21
22==1
2-0-=2=1
2--0=10-00--2-
1--1=011-2
1-100-2
11-1
112
222200110-11110-2=
1==2-210-1122=-2022
2==1-20-2-=-21
21=1-=0=-1-=-=-12
210---=-1===-===00
1--21=01-022=2-
11-=
21-2
2-22-0
1==-
1-----01
1-0-21-
211212-22-12--=2
1=220=10=20200-0
21-=11-==20=1
102-2==2-==-0
1=0=01-
22=1--0111
1-----=1==00===0-
20==-1-121-
1=--2--21-202=21
1-0122=--=
2==-2--===-=-1=-012
2220=-00-10-
1==0=---10212-1-
1221=-022120
100112
2-=0=220-=01-1=12-
1=00
2122121=2
2-==
12-21===10220
1=0
22-1=--01-==0
1-2-2202=1=02
11-===21-
1-22=0102-=-222-1
1021=0=21=0=11=2-2
1===2222=0-0=
20=2-=0=
1=2==02-1-100200
1=1211211----
12=1=1---=20
2--2=2=1100020200
1=0-002-11-10----
1=1=-1=-0
1-21=1=02=21=20
1-=0-002-0=--=0=
20-===-1=
220021012
12===1001=--=2=1-
1=-02
21
2=200222122
1-2=110111=012=1-
1=02=
101==1-=1=0001
1=22--1-021
120
201-0211
1-1012211--2=-1=
12=10202=20
11-22=2=2=-0==1=00
11=-
12-1--2222
22
1120--0==
10=-0=001-
1=00200200=
1-20
1-0-
10112-202=-011
11-2=
1-=10=1-0=-21==11200
11
12-0-=01--12
1-=1121-=0
120-21=-1=10121=
1102-22
21=
1==--1==---1-==
12=0--=2210-1==
22210=1-=0
1102-20-1
1=1=2102
11--=2==-0
2=0=1=
21-0-10=
12-212-2=-==102--
1==0100210-=
12210-0-1
1=0-2-0=
10-1=211=1-2=
201-0--
1--2=00=-1-211
121=-22100=210--
122-0-1-
2=211
2=1-=2-2-0001=-1
1102--11=0=1=10==2
1==01121-2--0=12
2=21-20=-0-0
2-212=21-02=1-2-02
2=1-0
10-0-=20
2=2=201";
