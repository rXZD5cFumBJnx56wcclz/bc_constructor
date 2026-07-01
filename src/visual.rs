pub const BT_SCRIPT: fn(&str) -> String = |symbol| {
    format!(
        r##"
    set datafile separator whitespace
    set datafile columnheaders
    set style fill solid
    set boxwidth 0.8
    set style textbox opaque fillcolor rgb "#EBEBEB" bordercolor rgb "#0F0F0F"
    plot \
    "data.dat" index 0 using "time":"open":"high":"low":"close" with candlesticks linecolor rgb "#7D2AD4" title "{symbol}", \
    "data.dat" index 1 using "time":"positions_orders" with lines linewidth 2 dashtype (40,10) linecolor rgb "#C2820C" title "positions_orders", \
    "data.dat" index 0 using "time":"entry" with points pointtype 7 pointsize 3 linecolor rgb "#0F0F0F" notitle, \
    "data.dat" index 0 using "time":"exit" with points lw 8 pointtype 2 pointsize 2 linecolor rgb "#0F0F0F" notitle, \
    "data.dat" index 0 using "time":"entry" with points pointtype 7 pointsize 2.5 linecolor rgb "#FFFFFF" notitle, \
    "data.dat" index 0 using "time":"exit" with points lw 6 pointtype 2 pointsize 2 linecolor rgb "#FFFFFF" notitle, \
    "data.dat" index 0 using "time":"entry" with points pointtype 7 pointsize 2 linecolor rgb "#00C222" title "entry", \
    "data.dat" index 0 using "time":"exit" with points lw 3 pointtype 2 pointsize 2 linecolor rgb "#C20006" title "exit", \
    "data.dat" index 0 using "time":(column("pnl") != column("pnl") ? NaN : column("open")):"pnl" with labels boxed offset 0,1 title "pnl", \
    "data.dat" index 0 using "time":(column("qty") != column("qty") ? NaN : column("open")):"pnl" with labels boxed offset 0,2 title "qty"
    "##,
    )
};
