const fs = require('fs');

var parse = require('csv-parse');

const csv = fs.readFileSync('./z80.csv').toString();

parse(csv, (err, inst) => {
  const opcodes = inst
    .map(extract);

  const json = JSON.stringify(opcodes, null, 2);
  fs.writeFileSync('./z80.json', json);
});



function extract([ name, time, , , opcode ]) {

  const set = {
    name,
    opcode,
  };

  if (name.includes(' ')) {
    const [ mn, operands ] = name.split(' ');
    Object.assign(set, {
      name: mn,
      operands: operands.split(','),
    });
  }

  return set;
}
