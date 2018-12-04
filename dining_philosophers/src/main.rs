/*
 * 昔々、裕福な慈善家が、5人の高名な哲学者が宿泊できるカレッジを寄付しました。
 * それぞれの哲学者には思索活動にふさわしい部屋が与えられました;
 * また共用のダイニングルームもあり、そこには丸いテーブルが置かれ、
 * 5人それぞれが専用で使うイス5脚で取り囲まれていました。
 * 彼らはテーブルを反時計回りに座ります.
 * 哲学者の左側にはそれぞれ金のフォークが配され、
 * 中央には大きなボウルに入ったスパゲッティが常に補充されていました。
 * 哲学者は大半の時間を思慮に費やすのですが;
 * 空腹になった時は、ダイニングルームに出向き、自分専用のイスに座り、
 * 左側のフォークを取上げ、スパゲッティに突き刺します。
 * しかし、絡まり合ったスパゲッティを口元まで運ぶには2本目のフォークが必要でした。
 * なので哲学者は自分の右側にあるフォークも使う必要がありました。
 * 食べ終わったら両側のフォークを元に戻し、席から立ちあがって、思索活動を続けます。
 * もちろん、1本のフォークは同時に1人の哲学者しか使えません。
 * 他の哲学者が食事したければ、 フォークが再び戻されるまで待たねばなりません。
 *
 */
 
 use std::thread;
 use std::time::Duration;
 use std::sync::{Mutex, Arc};
 
struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

impl Philosopher {
    // 引数でStringを取ってしまうと
    // 呼び出し元からまたto_string()を
    // 実行しなくてはいけなくなる。
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }
    
    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        thread::sleep(Duration::from_millis(150));
        let _right = table.forks[self.right].lock().unwrap();
        
        println!("{} is eating.", self.name);
        thread::sleep(Duration::from_millis(1000));
        println!("{} is done eating.", self.name);
    }
}

struct Table {
    forks: Vec<Mutex<()>>,
}

fn main() {
    let table = Arc::new(Table { forks: vec![
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
    ]});

    let philosophers = vec![
        Philosopher::new("Judith Butler", 0, 1),
        Philosopher::new("Gilles Deleuze", 1, 2),
        Philosopher::new("Karl Marx", 2, 3),
        Philosopher::new("Emma Goldman",3 ,4),
        Philosopher::new("Michel Foucault", 0, 4),
    ];
    
    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        let table = table.clone();
        
        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();
    
    for h in handles {
        h.join().unwrap();
    }
}
