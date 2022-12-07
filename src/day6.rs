use std::collections::HashSet;

use itertools::Itertools;

pub fn pb1() {
    dbg!(find_start_packet(INPUT));
}
pub fn pb2() {
    dbg!(find_start_message_performant(INPUT));
}

fn find_start_packet(input: &str) -> usize {
    const SIZE: usize = 4;
    let mut last_chars: [char; SIZE] = [' '; SIZE];
    for (i, char) in input.chars().enumerate() {
        last_chars[i % SIZE] = char;
        if i > SIZE - 1 && all_different_4(&last_chars) {
            return i + 1;
        }
    }
    panic!("didnt find start");
}
fn all_different_4(c: &[char; 4]) -> bool {
    c[0] != c[1] && c[0] != c[2] && c[0] != c[3] && c[1] != c[2] && c[1] != c[3] && c[2] != c[3]
}

#[allow(dead_code)]
fn find_start_message(input: &str) -> usize {
    const SIZE: usize = 14;
    let mut last_chars: [char; SIZE] = [' '; SIZE];
    for (i, char) in input.chars().enumerate() {
        last_chars[i % SIZE] = char;
        if i > SIZE - 1 && last_chars.iter().all_unique() {
            return i + 1;
        }
    }
    panic!("didnt find start of message");
}

#[allow(dead_code)]
fn find_start_message_alt(input: &str) -> usize {
    let message_position = input
        .as_bytes() // we need to turn in bytes as &str doesnt tell us if we are working byte per byte, char per char or grapheme
        .windows(14) // https://stackoverflow.com/questions/58770462/how-to-iterate-over-unicode-grapheme-clusters-in-rust
        .position(|window| window.iter().all_unique())
        .unwrap();
    message_position + 14
}

struct RingBuffer<'a> {
    data: &'a mut [char],
    pub size: usize,
    pub next_cursor: usize,
}

impl<'a> RingBuffer<'a> {
    fn insert(&mut self, c: char) {
        self.data[self.next_cursor] = c;
        self.next_cursor = (self.next_cursor + 1) % self.size;
    }

    fn iter(&self) -> std::ops::Range<usize> {
        (self.next_cursor)..(self.next_cursor + self.size)
    }

    fn get(&self, idx: usize) -> char {
        self.data[idx % self.size]
    }

    fn new(data: &'a mut [char]) -> Self {
        let size = data.len();
        Self {
            data,
            size,
            next_cursor: 0,
        }
    }
}

fn find_start_message_performant(input: &str) -> usize {
    const SIZE: usize = 14;
    let mut chars: [char; SIZE] = [' '; SIZE];
    let mut buffer = RingBuffer::new(&mut chars);
    let mut to_skip = SIZE + 1;
    for (pos, char) in input.chars().enumerate() {
        buffer.insert(char);
        to_skip -= 1;
        if to_skip == 0 {
            if let Some(skip) = find_dup_pos_right(&buffer) {
                to_skip = skip;
            } else {
                return pos + 1;
            }
        }
    }
    panic!("didnt find start of message");
}

fn find_dup_pos_right(buffer: &RingBuffer) -> Option<usize> {
    let mut seen: HashSet<char> = HashSet::new();
    for i in buffer.iter().rev() {
        let c = buffer.get(i);
        if seen.contains(&c) {
            return Some(i - buffer.next_cursor + 1);
        }
        seen.insert(c);
    }
    return None;
}
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_packet() {
        assert_eq!(find_start_packet("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(find_start_packet("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(find_start_packet("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(find_start_packet("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(find_start_packet("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
    #[test]
    fn test_message() {
        assert_eq!(find_start_message("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(find_start_message("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(find_start_message("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(find_start_message("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(find_start_message("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
    #[test]
    fn test_message_alt() {
        assert_eq!(find_start_message_alt("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(find_start_message_alt("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(find_start_message_alt("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(
            find_start_message_alt("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            29
        );
        assert_eq!(
            find_start_message_alt("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            26
        );
    }

    #[test]
    fn test_message_performant() {
        assert_eq!(
            find_start_message_performant("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
            19
        );
        assert_eq!(
            find_start_message_performant("bvwbjplbgvbhsrlpgdmjqwftvncz"),
            23
        );
        assert_eq!(
            find_start_message_performant("nppdvjthqldpwncqszvftbrmjlhg"),
            23
        );
        assert_eq!(
            find_start_message_performant("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            29
        );
        assert_eq!(
            find_start_message_performant("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            26
        );
    }
}

const INPUT: &str = "cdhccdbdggfjjgssjzjzggjnjpnpbbzbnzzflfjfnfrrpvrvbrvvrvggvlvnnbrnrcncsnndbndbnndbdndfdrdvrvvndvvbggnrrnbrnntffgttwzwnnmvmcvvhsstzzlnlwlttbzzpnpmnnjvjnntmnmfftwwrfwrwswmmfrrfrrgbrbffwvvshvhrhmhththbbmqbmqqlslhssrmmqdmmjtmtmjtmjtttnwnvwvqwqjjnbbbdbqbnbpnbnllglcglcgcdczdznnqhhfthtmtlldqlqrrmddrldlzdllvddjcddqfqbqsbqqnllwppqpqzzrbbdppzsppjdpdqpdqdfqfrrwbwrwwqcqcsqsvvpbvbbztzptzzpccdtdhdffvqvcvzzmwzwddjfdffplplqlvlmmmvggpmpvpddpbptplpvlplvpvvnrvnnbqqqjhhwfhwfhwhqhmmpphqpqvppfzpzjzddgzzwffjmjggwhwwnnmlmpmmhcmcpcrcddvzvpzzwnznfznffgdgvddvtvgvsvdsdbbjnjtntbnttgbbbvgvgrgrzrvzrzddlsddcndcnnfqnnmpmlppdlplzplzpzgzmzmddlvlnnbttbwwhbhdhfdfssjppmcpplpdddnpdnnljlwjljsjnjhnhvhvqqsffrbbdttjdjndjdwwsfspffnhfhhlvhvmmqjmmwzwszwwvdvpdvdbdtdsdtsshvvmtvmtmctclchccrllznzfffpjjvhhdmhhvphpghgsgmmhlhnlnmnlnslnlgngznnsqnqddllpwllmzmjmttptfpplglqlgglgqqptqqmvmtmjmddcchbblltslsvsmvmgghmmccnzcztczzmnmttrdrvvcvzvvzllbhllnldndbbqffbbgtgddbtdbbzttdptpccjnjppbllbzlblrlcllhrrhqhgqqbcqcvcdvvnnzfzvzttrptrrwmrmlrlddvttdbtdbdcdvccwlcwwhphmppwfppclpcllgqgnghhvfflfggrzrcchfhhrdhrdhdnhnmmhjjwqjjpmmwvmmdnmnzzqmqwwmtthtdhtthnnqhqdhqqndqqwffsbspbptpmmndnllsmmdhmhfhnhjhghshlslppbgpgngddlsljsjmmzqzhhswhssfzssfqqcmqcmqcmmqggjcjvvgssrccwddmpmwwdfdpdbdpdwdvvqfvfrvvvsbvbhvvmqqcjqqvzqzppncnhhqnnpgplpqqpjpbblpbbbshsthhvfhfmmqzmmznnvrvqrrwdrdlrlwlttzqttjvttqltqqnfqqqwjqwwqttfstftjffsqqnhhnsnqqhggbsgghfgglslmssqlqhlhthqhccdsspsnssshbbnmngnnhllwclcffqllsrszrssnqsqvqjvjcvcttqgqbqmmfqfsqfsqswwvcvffndnfdfvfcvvggsmsfmfwfpfwwzhznntgtlglmmlfmllwrlrwwhcchqchhznzjjcdjdbjjhcjcscwwlnnsgngqqtgqgngnwgnnhqnnhchmchhtchcnclcmccgffbmmzvvrnngwwvddzccnjntjtwjwwztwtmtddjddpsptpbpbvvbwwnlnmndmnmdnnclnnbsbddpfdfvvjtjqqtqqqzjzlzqllzzwwlppvfffpcffffprrncnnzsnznhhwvvqhqphpjjgqqvnnmdmqqglqlblgglrlsspscsjjpvpbpjjwccslsppdjpdjjwvjjmhjhtjjwqqbqjqzjqzqpqbbswwlssqzssbjjpjqjbbjcjpjspjssjjzhhhnjhnhbnhhwzhzwlcshqlqpzgggzmcwntcwmfgtrwwjdpnbdqqcgnzgbdrzdmpwgvtvqffqbpvjpjrcfswffllnvnwvhclpjcwqwgnwqwvwsfgflrgzzsswffwjdjgvdvlgmczcbthwbvhggwzwlzfmhvwvjpbpnhcczbgfhhgghsmjwnvnsvnvmqwstrgnncwbqgbqpgdngllcqnzgwswpgtwzgqzggnzsdgltrlqfctqfvlzdswccfpdtjbfnrbqsmpjclnplbmqbmvwbzzdflwbqrljvzjpcrmnqsmrpqlmfsgcmthqpwgwzvmrjnhqczljcpnzjbwzrhjrzmcqpmlbzhgmqrlzsjbjsvcmcngptzlrthwsrjrlmsrgjlzrvpzwmprwnpgvjtspsppfvwfwcvbnqcwwmzlbqthqmbnbmnsnzgsbbnqtrvhlzjhphclpjzrdblszrnftqgwwrhpznhjhgrncvsvrmtmmgssvzddjfrnrzhbrqrfffjvzrqdnrdbvjwgrvlcpbncfgczlwdggsjmwzhndcdbggjvwfljctjnsjwczwfdrfttbhnlswfdbpcnwpspdhnzwqbgdswwpccbpfpgmfmvvwpzbzqsbbjbfnhjpszcbnrdplmwtdjtpcsztdjcmczltnstzwlcdbtdhsdgsgtlvdfqggfmmrppjfrmtfwhpbjsppszjbhmthndqmvbmqcbtqsltwrcvlvblwspbgspjftwllzcmnsrvjpnstzrfmcflnhppsdfggwbzvnvlnjqlfvrlplnzvfrwvgcgqvnpfgtgchctvhcplclzmfpwgnfhqjgglfmsgpflqcpqmbbhwnvvdllcnhblpnndbdtmgvfbvvvlvzlrpfqmnvzbfrssjtlgcjtpfznshvdjrjnfshfcgvwcdbqlfsbhnzwmsgwhpbzttgfjsqgwvdmbdwjljhsndrbbzfrsqjhcbldzqpmtnfvnmzltjcrvrltwshnhqlnclmcnfpbzstsczlqmfmdftzfbcwqnhqppzfbzpbfjhmmtvtbmblmtshsbtjlvsqvmbmgstbbdmhprqmtpfdqqntmnlbmpsmwfgrvstjcllhwpcddnljdjvdrbwqmgrjnldpgnrhgqpzqrvwzsngrgnbpjnsffzjsbdptwnnfcqlscfhvggpfstsnqzcjbqqhgdpqsrlprcppgqmddpqpbnvgwtdqsbbgtvsqfrtqfsbdzhsztfmvwrrsjcbtcjgzrnhnpgldtwbwgmwbgmjjzsbbzlhgmlczrhjwtzrgwscmjvlstprldhglvftqzbtrmcwzgtjppbnjcdvjvcwvdbngnbrmjvvtnwdqfclbpgzcfnnnlnngtgmhsqsdmbjctjzjpbrwrhscqshmmwbtfnzjgsrjlnqqdsvdrjdzsdprphnfmwwcztqfcrjvnfhlvnqwbrfmcvhrbtgvcrqjjfcnzwmlfzzdcbbzvphhmsdltwjfdcgthpvszqzjdbfwrpvhbjqdhrscnvjhjvvcldnhgjclmzpbrrwnscgpcqrpdgsnjnwhctcdqgwqbrcszfzpmtdrhlftvwffdjrtznqrppqbdbwvzmtlpvsqqpcngjgfdrpngnspdwhhvlhqrtsphgqrlldggtrvqsprbfdmrpgcmqphdvjfmhlznpgtqlvtnllcdhzhhtjjlfvdlwhcrfmjmdjtmbllvsfgvmfqtqlmrlrjmqptszvjdpzhphppljnpjdjpwlrclssgdnstchhwhpflmlrtdqvqbbljrmnflrltzpqmgqfrczvfzrpfsrwsgpljvjfjdjdvjchcdmmtjgghqspwzdtwqgtvmnrrbfbgnhcrvnzznrdlqmgmdwmpwzlqdjtvpszwnjtjtmjqvfwvftlhgpvgzswpbvbllfcwpjnsmbhzrdpdzjsrpnhphdcqjmzvvhrjcwhgwjwcshqwzpbpmfnjjvqcjrqmvsrdrtdvfhwhrbpvrqrsfzflslqtdrtcsggtzmpvbszdgttlvpwwltvpcwqmnwqtpcfzgsvsmncvpqqdrljfwtncplmjlpfcnqmcctwzhrbmrfwvsrjsbnhjrjmrnbmmnnhsvlltwzzhsgwppnlmljgpcsmpchdjdzpgvrtwsfzffhnlbfmrldzbshvpqhnfzpwnvczgfvhbntcpztwqlfgtsmdhvcrgjhvqrhbpvbpzcpbgzrcfjztbnfjptbzfpztwprwf";
