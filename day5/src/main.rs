use itertools::Itertools;
use hashbrown::HashMap;

static INPUT: &str = "593,10 -> 593,98\n777,236 -> 964,236\n650,575 -> 476,575\n120,612 -> 715,17\n508,707 -> 508,89\n98,834 -> 751,834\n623,554 -> 623,701\n929,976 -> 62,109\n368,893 -> 330,931\n495,335 -> 40,335\n44,704 -> 423,704\n683,711 -> 683,487\n26,940 -> 833,133\n961,183 -> 454,183\n301,306 -> 301,935\n973,822 -> 398,822\n639,911 -> 515,911\n861,180 -> 184,857\n31,97 -> 857,923\n966,376 -> 966,114\n881,485 -> 881,377\n930,98 -> 110,918\n841,889 -> 841,35\n512,261 -> 880,261\n48,533 -> 48,674\n207,226 -> 52,226\n823,952 -> 177,306\n331,566 -> 423,566\n422,418 -> 422,130\n699,517 -> 699,567\n757,784 -> 241,784\n508,445 -> 560,393\n866,275 -> 435,706\n74,41 -> 74,258\n386,369 -> 334,317\n240,94 -> 240,969\n851,197 -> 577,197\n28,906 -> 741,193\n286,227 -> 286,293\n849,800 -> 849,665\n736,307 -> 336,307\n69,701 -> 494,276\n421,823 -> 96,823\n121,626 -> 121,393\n318,351 -> 194,351\n670,671 -> 439,671\n603,914 -> 603,272\n61,507 -> 61,889\n266,39 -> 157,39\n543,664 -> 869,664\n382,709 -> 884,709\n499,80 -> 548,80\n489,79 -> 878,79\n695,86 -> 644,86\n987,585 -> 987,557\n287,67 -> 551,67\n975,983 -> 35,43\n707,351 -> 232,351\n529,175 -> 852,175\n32,811 -> 604,811\n106,153 -> 815,153\n195,268 -> 509,582\n50,922 -> 312,922\n220,500 -> 872,500\n473,33 -> 569,33\n858,847 -> 162,151\n937,947 -> 26,36\n726,435 -> 402,435\n686,601 -> 474,813\n764,880 -> 84,200\n850,950 -> 850,464\n413,620 -> 413,285\n893,560 -> 229,560\n149,100 -> 149,901\n358,613 -> 243,613\n202,445 -> 202,411\n127,153 -> 513,539\n147,846 -> 53,940\n139,920 -> 679,380\n913,953 -> 913,735\n339,466 -> 339,177\n113,882 -> 647,882\n18,880 -> 134,880\n897,152 -> 897,428\n473,511 -> 636,511\n880,370 -> 358,370\n400,244 -> 721,244\n419,987 -> 120,688\n872,224 -> 481,224\n335,302 -> 730,302\n961,324 -> 961,157\n769,301 -> 959,301\n829,124 -> 144,124\n523,372 -> 985,372\n520,33 -> 520,685\n554,644 -> 808,898\n82,676 -> 870,676\n303,612 -> 303,705\n338,40 -> 338,939\n836,47 -> 72,811\n371,751 -> 575,955\n929,505 -> 929,324\n273,181 -> 275,183\n347,595 -> 347,463\n95,629 -> 95,606\n809,188 -> 126,871\n857,924 -> 145,212\n668,277 -> 668,63\n700,904 -> 700,45\n814,899 -> 22,899\n205,98 -> 714,607\n943,28 -> 40,931\n282,620 -> 773,129\n424,803 -> 285,803\n688,329 -> 299,329\n146,628 -> 34,628\n573,417 -> 164,826\n292,232 -> 412,112\n412,508 -> 145,508\n632,648 -> 632,92\n885,904 -> 885,513\n295,981 -> 132,818\n134,681 -> 41,681\n810,531 -> 959,531\n188,590 -> 188,215\n960,795 -> 189,24\n729,211 -> 729,833\n214,817 -> 845,817\n196,609 -> 584,609\n384,908 -> 384,101\n770,907 -> 770,530\n451,469 -> 451,812\n571,261 -> 834,261\n799,436 -> 799,983\n248,105 -> 248,879\n783,906 -> 783,903\n955,670 -> 790,670\n723,750 -> 723,429\n572,427 -> 546,427\n610,341 -> 527,341\n925,426 -> 816,317\n151,403 -> 151,684\n408,969 -> 408,369\n276,425 -> 276,75\n186,86 -> 186,758\n412,420 -> 412,531\n361,60 -> 976,60\n787,649 -> 667,769\n45,866 -> 91,866\n319,963 -> 51,963\n112,866 -> 112,747\n291,475 -> 504,475\n175,116 -> 357,116\n968,961 -> 968,213\n13,12 -> 987,986\n640,728 -> 767,728\n981,505 -> 246,505\n864,981 -> 128,981\n91,66 -> 931,906\n798,116 -> 91,823\n552,74 -> 88,538\n620,872 -> 232,872\n45,229 -> 658,229\n413,75 -> 413,436\n815,257 -> 815,686\n989,22 -> 36,975\n178,904 -> 233,849\n635,128 -> 635,96\n640,820 -> 640,313\n890,787 -> 167,64\n221,22 -> 826,22\n914,132 -> 60,986\n848,31 -> 392,487\n105,969 -> 858,969\n903,868 -> 143,108\n38,941 -> 621,358\n171,340 -> 14,497\n286,460 -> 81,255\n726,688 -> 857,819\n494,689 -> 510,689\n517,913 -> 598,913\n932,66 -> 932,431\n977,982 -> 18,23\n95,101 -> 95,278\n574,467 -> 349,467\n63,803 -> 63,882\n838,874 -> 255,874\n900,752 -> 181,33\n102,897 -> 989,10\n374,439 -> 374,277\n513,504 -> 513,885\n814,932 -> 814,407\n824,656 -> 959,521\n415,570 -> 616,570\n577,880 -> 577,181\n287,524 -> 986,524\n955,665 -> 323,665\n556,365 -> 263,658\n154,226 -> 886,226\n803,750 -> 866,750\n558,725 -> 558,395\n941,115 -> 941,150\n180,410 -> 180,874\n458,753 -> 112,753\n199,253 -> 363,253\n423,650 -> 22,650\n892,851 -> 279,238\n611,109 -> 611,198\n983,344 -> 339,988\n299,47 -> 299,934\n435,652 -> 700,387\n186,775 -> 677,284\n136,576 -> 136,368\n818,744 -> 305,744\n767,171 -> 767,431\n930,842 -> 259,171\n342,831 -> 342,601\n193,672 -> 46,525\n925,164 -> 528,164\n725,92 -> 617,200\n67,729 -> 67,739\n547,153 -> 547,245\n763,434 -> 763,509\n314,888 -> 357,888\n72,645 -> 491,645\n92,67 -> 240,67\n827,936 -> 788,897\n852,378 -> 77,378\n448,337 -> 668,337\n846,739 -> 499,739\n465,691 -> 315,541\n716,163 -> 18,861\n78,965 -> 983,60\n114,952 -> 820,246\n950,351 -> 419,882\n266,36 -> 266,482\n773,841 -> 773,66\n742,198 -> 742,46\n417,512 -> 304,625\n900,277 -> 900,338\n983,431 -> 473,941\n986,282 -> 734,30\n742,19 -> 769,19\n952,320 -> 948,324\n92,590 -> 548,590\n107,39 -> 107,696\n603,749 -> 603,26\n55,282 -> 888,282\n670,848 -> 985,533\n981,982 -> 92,93\n147,428 -> 649,930\n773,737 -> 821,785\n791,576 -> 791,852\n327,672 -> 530,469\n847,122 -> 381,122\n419,493 -> 498,572\n879,842 -> 879,239\n267,717 -> 267,869\n142,449 -> 174,417\n342,718 -> 342,397\n603,207 -> 314,207\n612,648 -> 735,771\n37,10 -> 971,944\n891,716 -> 891,86\n252,217 -> 662,627\n185,165 -> 941,921\n854,717 -> 676,717\n158,791 -> 336,791\n762,226 -> 98,890\n73,189 -> 92,189\n649,511 -> 253,115\n719,456 -> 514,251\n605,286 -> 325,286\n454,609 -> 454,489\n374,541 -> 783,541\n599,177 -> 94,682\n600,384 -> 32,384\n810,933 -> 39,162\n780,871 -> 409,871\n24,639 -> 24,316\n454,80 -> 454,95\n556,541 -> 907,541\n627,295 -> 750,295\n245,71 -> 214,102\n725,445 -> 614,445\n779,538 -> 779,390\n746,667 -> 351,272\n117,776 -> 117,660\n498,495 -> 88,905\n697,721 -> 697,919\n580,314 -> 580,166\n22,656 -> 641,37\n413,433 -> 44,802\n182,305 -> 805,928\n739,277 -> 739,499\n172,210 -> 172,259\n894,576 -> 894,322\n265,263 -> 265,437\n430,228 -> 780,578\n464,531 -> 798,531\n713,63 -> 668,63\n918,831 -> 256,169\n414,375 -> 467,375\n440,32 -> 391,32\n439,806 -> 955,806\n335,820 -> 335,279\n727,458 -> 422,458\n312,274 -> 619,581\n136,724 -> 538,322\n589,680 -> 589,850\n335,648 -> 232,545\n499,216 -> 405,216\n942,710 -> 942,455\n969,556 -> 721,556\n756,552 -> 756,902\n98,870 -> 445,870\n476,833 -> 476,269\n820,127 -> 407,127\n337,519 -> 714,519\n756,95 -> 11,840\n317,339 -> 317,286\n353,86 -> 43,86\n93,950 -> 938,105\n705,509 -> 705,319\n244,879 -> 721,402\n434,794 -> 711,517\n272,381 -> 431,381\n652,104 -> 652,587\n850,866 -> 34,50\n645,902 -> 79,336\n701,39 -> 701,295\n492,793 -> 95,396\n352,554 -> 395,554\n123,405 -> 322,206\n941,745 -> 716,520\n450,512 -> 569,631\n42,25 -> 817,800\n909,387 -> 909,863\n919,934 -> 919,546\n439,881 -> 569,881\n167,866 -> 167,669\n242,264 -> 242,694\n981,786 -> 228,33\n452,434 -> 452,660\n22,26 -> 22,29\n26,155 -> 677,806\n801,627 -> 313,627\n657,135 -> 657,270\n872,875 -> 440,443\n636,248 -> 636,338\n776,51 -> 93,51\n498,600 -> 894,600\n263,984 -> 263,807\n416,390 -> 899,873\n269,137 -> 976,137\n752,12 -> 752,617\n55,925 -> 548,925\n856,551 -> 771,551\n653,93 -> 653,587\n403,286 -> 403,417\n895,706 -> 221,32\n139,822 -> 139,928\n696,194 -> 696,143\n270,678 -> 710,678\n879,353 -> 879,360\n949,712 -> 752,712\n665,661 -> 817,661\n462,952 -> 980,434\n692,766 -> 692,478\n157,117 -> 144,117\n438,701 -> 408,701\n401,703 -> 401,724\n876,831 -> 108,63\n749,892 -> 832,892\n455,124 -> 455,776\n551,222 -> 551,372\n533,80 -> 726,80\n342,740 -> 56,740\n793,370 -> 34,370\n949,614 -> 949,623\n610,287 -> 610,760\n978,834 -> 85,834\n644,894 -> 644,341\n35,887 -> 176,887\n168,958 -> 964,162\n341,886 -> 341,470\n417,845 -> 417,702\n338,347 -> 304,313\n651,10 -> 72,10\n853,160 -> 853,85\n381,568 -> 436,623\n794,437 -> 250,437\n861,72 -> 206,72\n807,813 -> 807,827\n820,502 -> 820,329\n547,508 -> 547,773\n160,129 -> 160,175\n756,468 -> 756,80\n442,661 -> 405,661\n304,817 -> 304,765\n99,42 -> 957,900\n212,110 -> 854,752\n44,620 -> 661,620\n212,311 -> 784,883\n329,671 -> 329,908\n86,359 -> 553,826\n257,799 -> 934,122\n409,663 -> 409,367\n528,623 -> 593,688\n957,525 -> 544,938\n846,766 -> 113,33\n176,680 -> 176,102\n167,287 -> 167,929\n932,870 -> 834,968\n86,774 -> 49,774\n745,231 -> 70,906\n435,760 -> 138,463\n776,810 -> 625,810\n928,930 -> 76,78\n602,24 -> 602,688\n394,424 -> 65,424\n946,966 -> 93,113\n494,39 -> 951,39\n607,699 -> 832,699\n13,403 -> 391,403\n726,475 -> 726,29\n828,625 -> 836,617\n396,770 -> 167,770\n28,546 -> 374,200\n56,113 -> 837,894\n290,589 -> 740,139\n930,805 -> 296,171\n646,895 -> 49,895\n111,15 -> 111,497\n11,274 -> 570,833\n257,624 -> 603,624\n63,844 -> 666,844\n846,661 -> 846,464\n431,72 -> 431,674\n726,674 -> 726,40\n286,660 -> 286,909\n847,222 -> 847,861\n325,896 -> 325,416\n793,953 -> 365,953\n987,956 -> 62,31\n845,853 -> 363,371\n79,782 -> 506,782\n424,21 -> 424,369\n938,162 -> 177,923\n86,193 -> 799,906\n320,164 -> 320,654\n840,306 -> 840,711\n852,736 -> 852,690\n876,966 -> 143,233\n787,926 -> 38,177\n374,112 -> 340,112\n132,541 -> 740,541\n29,28 -> 968,967\n916,212 -> 170,958\n371,553 -> 521,403\n88,796 -> 870,796\n656,367 -> 71,367\n785,166 -> 785,427\n320,30 -> 320,549\n909,527 -> 816,620\n832,965 -> 302,965\n672,259 -> 80,259\n578,513 -> 578,243\n975,561 -> 537,123\n135,330 -> 188,330\n501,695 -> 501,573\n717,230 -> 878,230\n854,501 -> 27,501\n705,885 -> 950,885\n704,338 -> 704,630\n477,485 -> 864,485\n901,42 -> 305,638\n660,540 -> 660,546\n555,79 -> 190,79\n226,126 -> 800,700\n575,908 -> 944,908\n94,478 -> 94,746\n461,425 -> 929,893\n861,429 -> 451,19\n832,825 -> 179,172\n186,133 -> 298,133\n684,270 -> 558,270\n786,872 -> 125,872\n649,178 -> 649,595\n893,738 -> 412,257\n760,854 -> 901,713\n16,914 -> 866,64\n935,928 -> 266,259\n323,229 -> 32,229\n608,828 -> 608,49\n715,892 -> 74,251\n787,187 -> 787,903\n405,793 -> 405,183\n232,704 -> 232,389\n130,706 -> 130,657";

fn num_overlapping(lines: impl Iterator<Item=(i32,i32,i32,i32)>) -> usize {
  let mut points = HashMap::new();
  for (x1,y1,x2,y2) in lines {
    let dx = (x2 - x1).signum();
    let dy = (y2 - y1).signum();
    let (mut x, mut y) = (x1,y1);
    while (x,y) != (x2+dx,y2+dy) {
      *points.entry((x,y)).or_insert(0) += 1;
      x += dx;
      y += dy;
    }
  }
  points.values().filter(|&&n| n > 1).count()
}

fn main() {
  let lines = INPUT.lines()
    .filter_map(|l| l.split(" -> ")
      .map(|s| s.split(','))
      .flatten()
      .map(|i| i.parse().unwrap())
      .collect_tuple()
    )
    .collect::<Vec<_>>();
  let p1 = num_overlapping(lines.iter().copied().filter(|(x1,y1,x2,y2)| x1 == x2 || y1 == y2));
  let p2 = num_overlapping(lines.iter().copied());
  println!("{:?}", (p1,p2));
}
