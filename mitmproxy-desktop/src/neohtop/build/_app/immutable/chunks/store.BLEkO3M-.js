import{af as E,a2 as T,ag as V,x as O,ah as q,X as B,ac as C,J as $,ai as R,w as z,aj as F,ak as J,al as L,R as y,A as D,a as X,z as b,am as x,an as G,ao as K,ap as Q,aq as U,ar as Z,v as ee,M as re,p as te,h as S,l as ae,c as ne,n as N,d as se,a9 as ie,g as ue,F as oe}from"./runtime.ClIWe6PW.js";import{d as fe}from"./disclose-version.CVYayuu6.js";let I=!1;function ce(){I||(I=!0,document.addEventListener("reset",e=>{Promise.resolve().then(()=>{var r;if(!e.defaultPrevented)for(const a of e.target.elements)(r=a.__on_r)==null||r.call(a)})},{capture:!0}))}function H(e){var r=V,a=O;E(null),T(null);try{return e()}finally{E(r),T(a)}}function ye(e,r,a,n=a){e.addEventListener(r,()=>H(a));const s=e.__on_r;s?e.__on_r=()=>{s(),n()}:e.__on_r=n,ce()}const le=new Set,M=new Set;function de(e,r,a,n){function s(t){if(n.capture||p.call(r,t),!t.cancelBubble)return H(()=>a.call(this,t))}return e.startsWith("pointer")||e.startsWith("touch")||e==="wheel"?B(()=>{r.addEventListener(e,s,n)}):r.addEventListener(e,s,n),s}function we(e,r,a,n,s){var t={capture:n,passive:s},o=de(e,r,a,t);(r===document.body||r===window||r===document)&&q(()=>{r.removeEventListener(e,o,t)})}function p(e){var k;var r=this,a=r.ownerDocument,n=e.type,s=((k=e.composedPath)==null?void 0:k.call(e))||[],t=s[0]||e.target,o=0,v=e.__root;if(v){var l=s.indexOf(v);if(l!==-1&&(r===document||r===window)){e.__root=r;return}var d=s.indexOf(r);if(d===-1)return;l<=d&&(o=l)}if(t=s[o]||e.target,t!==r){C(e,"currentTarget",{configurable:!0,get(){return t||a}});var m=V,f=O;E(null),T(null);try{for(var i,u=[];t!==null;){var c=t.assignedSlot||t.parentNode||t.host||null;try{var _=t["__"+n];if(_!==void 0&&!t.disabled)if($(_)){var[Y,...j]=_;Y.apply(t,[e,...j])}else _.call(t,e)}catch(g){i?u.push(g):i=g}if(e.cancelBubble||c===r||c===null)break;t=c}if(i){for(let g of u)queueMicrotask(()=>{throw g});throw i}}finally{e.__root=r,delete e.currentTarget,E(m),T(f)}}}const _e=["touchstart","touchmove"];function ve(e){return _e.includes(e)}let P=!0;function Ee(e,r){var a=r==null?"":typeof r=="object"?r+"":r;a!==(e.__t??(e.__t=e.nodeValue))&&(e.__t=a,e.nodeValue=a==null?"":a+"")}function he(e,r){return W(e,r)}function Te(e,r){R(),r.intro=r.intro??!1;const a=r.target,n=S,s=b;try{for(var t=z(a);t&&(t.nodeType!==8||t.data!==F);)t=J(t);if(!t)throw L;y(!0),D(t),X();const o=W(e,{...r,anchor:t});if(b===null||b.nodeType!==8||b.data!==x)throw G(),L;return y(!1),o}catch(o){if(o===L)return r.recover===!1&&K(),R(),Q(a),y(!1),he(e,r);throw o}finally{y(n),D(s)}}const h=new Map;function W(e,{target:r,anchor:a,props:n={},events:s,context:t,intro:o=!0}){R();var v=new Set,l=f=>{for(var i=0;i<f.length;i++){var u=f[i];if(!v.has(u)){v.add(u);var c=ve(u);r.addEventListener(u,p,{passive:c});var _=h.get(u);_===void 0?(document.addEventListener(u,p,{passive:c}),h.set(u,1)):h.set(u,_+1)}}};l(U(le)),M.add(l);var d=void 0,m=Z(()=>{var f=a??r.appendChild(ee());return re(()=>{if(t){te({});var i=ne;i.c=t}s&&(n.$$events=s),S&&fe(f,null),P=o,d=e(f,n)||{},P=!0,S&&(O.nodes_end=b),t&&ae()}),()=>{var c;for(var i of v){r.removeEventListener(i,p);var u=h.get(i);--u===0?(document.removeEventListener(i,p),h.delete(i)):h.set(i,u)}M.delete(l),A.delete(d),f!==a&&((c=f.parentNode)==null||c.removeChild(f))}});return A.set(d,m),d}let A=new WeakMap;function me(e){const r=A.get(e);r&&r()}function be(e,r,a){if(e==null)return r(void 0),N;const n=se(()=>e.subscribe(r,a));return n.unsubscribe?()=>n.unsubscribe():n}let w=!1;function Le(e,r,a){const n=a[r]??(a[r]={store:null,source:ie(void 0),unsubscribe:N});if(n.store!==e)if(n.unsubscribe(),n.store=e??null,e==null)n.source.v=void 0,n.unsubscribe=N;else{var s=!0;n.unsubscribe=be(e,t=>{s?n.source.v=t:oe(n.source,t)}),s=!1}return ue(n.source)}function Re(){const e={};return q(()=>{for(var r in e)e[r].unsubscribe()}),e}function Se(e,r,a){return e.set(a),r}function Ne(e){var r=w;try{return w=!1,[e(),w]}finally{w=r}}export{Ee as a,Le as b,Ne as c,ce as d,P as e,we as f,Se as g,Te as h,ye as l,he as m,Re as s,me as u};