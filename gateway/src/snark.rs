use rand::Rng;

pub fn random_snark(author: &str) -> String {
    let mut rng = rand::thread_rng();
    let key = rng.gen_range(0..=60);
    snark(key, author)
}

pub fn snark(key: u8, author: &str) -> String {
    let snark = match key {
        0 => "{}, brevity is the soul of wit.",
        1 => "Nice diatribe {}.",
        2 => "I'm sure we all appreciate the a full-length thesis, {}.",
        3 => "That opinion is longer than a CVS receipt {}.",
        4 => "Patience is a virtue, unless you're waiting for {}'s opinions to end.",
        5 => "{}'s opinions are like a never-ending marathon, only without the medals.",
        6 => "I could've listened to all of Beethoven's symphonies in the time it took to read {}'s views.",
        7 => "Nice filibuster {}.",
        8 => "I think I've aged a decade waiting for {} to get to the point.",
        9 => "I've seen paint dry faster than {} can express an opinion.",
        10 => "Reading {}'s take is like watching paint dry—only slower.",
        11 => "{}'s opinion: the literary equivalent of a marathon.",
        12 => "I lost track of where I was in {}'s poor take. Twice.",
        13 => "{}'s take: perfect bedtime reading if you struggle with insomnia.",
        14 => "{}'s opinion: the literary equivalent of a never-ending loop of \"The Macarena.\"",
        15 => "By the time you finish reading {}'s opinion, you'll be in a rocking chair.",
        16 => "I have more patience waiting for paint to dry.",
        17 => "Patience, young grasshopper, {}'s magnum opus is still loading.",
        18 => "We all appreciate the novel that you've written {}.",
        19 => "I have time to meditate, do yoga, and finish my to-do list while {} is composing.",
        20 => "{} could probably write a dissertation on why he didn't like a bag of chips.",
        21 => "I have a feeling the next installment of {}'s opinion will be delivered via carrier pigeon.",
        22 => "{}'s opinion is like an infinite scroll, just when you think you're nearing the end, there's more.",
        23 => "{}'s opinions would make a decent doorstop",
        24 => "Time dilation occurs in the presence of {}'s verbose outpourings.",
        25 => "Eternity has a shorter shelf-life than {}'s diatribes.",
        26 => "I'm not sure if your opinion is a novel or a manifesto.",
        27 => "Your opinion is so long, I need a GPS to navigate through it.",
        28 => "Your opinion is like a lazy river: it just keeps going on and on.",
        29 => "Your opinion is longer than the line at the DMV on a Monday morning.",
        30 => "If I had a nickel for every word in your opinion, I'd be richer than Bezos.",
        31 => "{} are you paid by the word?",
        32 => "Could you say that in more words {}?",
        33 => "{} is ranting again.",
        34 => "I could read \"War and Peace\" twice while I wait for you to wrap up.",
        35 => "By the time I finish reading this, my grandkids will be old enough to read it to me.",
        36 => "I'm starting to think you're writing a novel disguised as an opinion.",
        37 => "If this were a movie, the intermission would have come and gone by now.",
        38 => "Your opinion is like a marathon: long, arduous, and I just want it to be over.",
        39 => "{}'s opinions could be used as toilet paper, but the roll would be comically large.",
        40 => "I'd rather listen to a toddler's bedtime story on repeat than endure {}'s tirades.",
        41 => "{}'s opinions are like a dying whale, endlessly circling around while taking forever to finish.",
        42 => "{}'s opinions make the \"War and Peace\" seem like a concise haiku.",
        43 => "If {}'s opinions were food, they'd be an all-you-can-eat buffet with endless courses.",
        44 => "I'd need a vacation to recover from reading just one of {}'s marathon opinions.",
        45 => "The length of this opinion is inversely proportional to its coherence.",
        46 => "I'd rather watch paint dry than read this interminable diatribe.",
        47 => "\"Brevity is the soul of wit.\" - William Shakespeare (apparently not {})",
        48 => "{}'s takes: The literary equivalent of a root canal.",
        49 => "I'm considering using your take as a doorstop.",
        50 => "{}'s opinion is like their toenails: long and overdue for a trim.",
        51 => "I could make a sandwich, watch a movie, and take a nap before {}'s take wraps up.",
        52 => "{}'s opinion is like a mathematical limit: it exists, but it's beyond my comprehension.",
        53 => "I've seen trees grow faster than {} can wrap up a take.",
        54 => "Your take has more words than a Stephen King novel.",
        55 => "Is this supposed to be a bedtime story or a history textbook?",
        56 => "This is like trying to read through a dictionary... except less interesting.",
        57 => "Can I get a mid-essay snack before I continue?",
        58 => "I'm beginning to suspect {} is trying to exhaust me into submission.",
        59 => "I started reading {}'s opinion when I was 12. I'm 35 now.",
        60 => "I once started reading a {} take, by the time I looked up my grandchildren were asking me to tell them bedtime stories.",
        _ => "{}, word economy is a thing.",
    };
    snark.replace("{}", author)
}
