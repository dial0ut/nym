import React from 'react';

import { TermsModal } from './TermsModal';

export default {
  title: 'Buy/TermsModal',
  component: TermsModal,
};

const terms =
  'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Philosophi autem in suis lectulis plerumque moriuntur. Piso igitur hoc modo, vir optimus tuique, ut scis, amantissimus. At ille pellit, qui permulcet sensum voluptate. Rhetorice igitur, inquam, nos mavis quam dialectice disputare? Consequens enim est et post oritur, ut dixi. Hoc mihi cum tuo fratre convenit. Conclusum est enim contra Cyrenaicos satis acute, nihil ad Epicurum. Duo Reges: constructio interrete. Id mihi magnum videtur.\n' +
  '\n' +
  'Quae duo sunt, unum facit. Moriatur, inquit. Quod non faceret, si in voluptate summum bonum poneret. Nunc haec primum fortasse audientis servire debemus.\n' +
  '\n' +
  'Aliud igitur esse censet gaudere, aliud non dolere. Quodsi ipsam honestatem undique pertectam atque absolutam. Tum ille timide vel potius verecunde: Facio, inquit. Primum Theophrasti, Strato, physicum se voluit;\n' +
  '\n' +
  'Apparet statim, quae sint officia, quae actiones. Hinc ceteri particulas arripere conati suam quisque videro voluit afferre sententiam. Quid de Pythagora? Quodsi ipsam honestatem undique pertectam atque absolutam. Cuius quidem, quoniam Stoicus fuit, sententia condemnata mihi videtur esse inanitas ista verborum. Eademne, quae restincta siti? Omnis enim est natura diligens sui.\n' +
  '\n' +
  'Nec enim, omnes avaritias si aeque avaritias esse dixerimus, sequetur ut etiam aequas esse dicamus. Et non ex maxima parte de tota iudicabis? Cur iustitia laudatur?\n';

export const Terms = () => (
  <TermsModal
    termsText={terms}
    lastUpdated={Date.now()}
    onAccept={async () => console.log('user has accepted')}
    onDecline={async () => console.log('user has declined')}
    onClose={async () => console.log('closed')}
  />
);
