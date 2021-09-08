exports._add = n1 => n2 => n1 + n2;
exports._sub = n1 => n2 => n1 - n2;
exports._mul = n1 => n2 => n1 * n2;
exports.div = n1 => n2 => n1 / n2;
exports._eq = n1 => n2 => n1 === n2;
exports._compare = n1 => n2 => n1.compare(n2);
exports._fromString = just => nothing => s => {
    try {
        return just(BigInt(s))
    } catch (_) {
        return nothing
    }
};

exports.toString = radix => i => i.toString(radix);
exports.toNumber = n => Number(n);
exports.fromInt = i => BigInt(i);
exports.mod = n1 => n2 => n1 % n2;
