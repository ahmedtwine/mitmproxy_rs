const __vite__mapDeps=(i,m=__vite__mapDeps,d=(m.f||(m.f=["../nodes/0.NQgD_XHH.js","../chunks/disclose-version.CVYayuu6.js","../chunks/runtime.ClIWe6PW.js","../chunks/legacy.C1eb0qk7.js","../chunks/slot.CWqy8z2H.js","../assets/0.C6iJBuhP.css","../nodes/1.BIAsjNCt.js","../chunks/store.BLEkO3M-.js","../chunks/lifecycle.BE0WvH52.js","../chunks/entry.peMtFI_s.js","../chunks/index.CIebrN9h.js","../nodes/2.nEq3Hs-V.js","../chunks/index-client.Dut3aJz6.js","../assets/2.Byf_auBo.css"])))=>i.map(i=>d[i]);
var V=n=>{throw TypeError(n)};var N=(n,e,r)=>e.has(n)||V("Cannot "+r);var l=(n,e,r)=>(N(n,e,"read from private field"),r?r.call(n):e.get(n)),A=(n,e,r)=>e.has(n)?V("Cannot add the same private member more than once"):e instanceof WeakSet?e.add(n):e.set(n,r),C=(n,e,r,a)=>(N(n,e,"write to private field"),a?a.call(n,r):e.set(n,r),r);import{h as U,a as J,K as M,O as Q,M as X,z as Y,N as Z,g as v,F as w,ab as p,ac as $,a9 as ee,p as te,u as re,b as se,ad as ne,k,l as ae,ae as L,q as oe,m as ce,t as ie,o as le,i as O}from"../chunks/runtime.ClIWe6PW.js";import{h as ue,m as fe,u as de,a as me}from"../chunks/store.BLEkO3M-.js";import{c as T,a as P,t as K,b as he}from"../chunks/disclose-version.CVYayuu6.js";import{p as j,o as _e,a as ve,i as q,b as B}from"../chunks/index-client.Dut3aJz6.js";function D(n,e,r){U&&J();var a=n,o,i;M(()=>{o!==(o=e())&&(i&&(Z(i),i=null),o&&(i=X(()=>r(a,o))))},Q),U&&(a=Y)}function ge(n){return class extends ye{constructor(e){super({component:n,...e})}}}var g,f;class ye{constructor(e){A(this,g);A(this,f);var i;var r=new Map,a=(s,t)=>{var d=ee(t);return r.set(s,d),d};const o=new Proxy({...e.props||{},$$events:{}},{get(s,t){return v(r.get(t)??a(t,Reflect.get(s,t)))},has(s,t){return v(r.get(t)??a(t,Reflect.get(s,t))),Reflect.has(s,t)},set(s,t,d){return w(r.get(t)??a(t,d),d),Reflect.set(s,t,d)}});C(this,f,(e.hydrate?ue:fe)(e.component,{target:e.target,anchor:e.anchor,props:o,context:e.context,intro:e.intro??!1,recover:e.recover})),(!((i=e==null?void 0:e.props)!=null&&i.$$host)||e.sync===!1)&&p(),C(this,g,o.$$events);for(const s of Object.keys(l(this,f)))s==="$set"||s==="$destroy"||s==="$on"||$(this,s,{get(){return l(this,f)[s]},set(t){l(this,f)[s]=t},enumerable:!0});l(this,f).$set=s=>{Object.assign(o,s)},l(this,f).$destroy=()=>{de(l(this,f))}}$set(e){l(this,f).$set(e)}$on(e,r){l(this,g)[e]=l(this,g)[e]||[];const a=(...o)=>r.call(this,...o);return l(this,g)[e].push(a),()=>{l(this,g)[e]=l(this,g)[e].filter(o=>o!==a)}}$destroy(){l(this,f).$destroy()}}g=new WeakMap,f=new WeakMap;const be="modulepreload",Ee=function(n,e){return new URL(n,e).href},z={},F=function(e,r,a){let o=Promise.resolve();if(r&&r.length>0){const s=document.getElementsByTagName("link"),t=document.querySelector("meta[property=csp-nonce]"),d=(t==null?void 0:t.nonce)||(t==null?void 0:t.getAttribute("nonce"));o=Promise.allSettled(r.map(u=>{if(u=Ee(u,a),u in z)return;z[u]=!0;const y=u.endsWith(".css"),x=y?'[rel="stylesheet"]':"";if(!!a)for(let m=s.length-1;m>=0;m--){const _=s[m];if(_.href===u&&(!y||_.rel==="stylesheet"))return}else if(document.querySelector(`link[href="${u}"]${x}`))return;const c=document.createElement("link");if(c.rel=y?"stylesheet":be,y||(c.as="script"),c.crossOrigin="",c.href=u,d&&c.setAttribute("nonce",d),document.head.appendChild(c),y)return new Promise((m,_)=>{c.addEventListener("load",m),c.addEventListener("error",()=>_(new Error(`Unable to preload CSS for ${u}`)))})}))}function i(s){const t=new Event("vite:preloadError",{cancelable:!0});if(t.payload=s,window.dispatchEvent(t),!t.defaultPrevented)throw s}return o.then(s=>{for(const t of s||[])t.status==="rejected"&&i(t.reason);return e().catch(i)})},Oe={};var Pe=K('<div id="svelte-announcer" aria-live="assertive" aria-atomic="true" style="position: absolute; left: 0; top: 0; clip: rect(0 0 0 0); clip-path: inset(50%); overflow: hidden; white-space: nowrap; width: 1px; height: 1px"><!></div>'),Re=K("<!> <!>",1);function ke(n,e){te(e,!0);let r=j(e,"components",23,()=>[]),a=j(e,"data_0",3,null),o=j(e,"data_1",3,null);re(()=>e.stores.page.set(e.page)),se(()=>{e.stores,e.page,e.constructors,r(),e.form,a(),o(),e.stores.page.notify()});let i=L(!1),s=L(!1),t=L(null);_e(()=>{const b=e.stores.page.subscribe(()=>{v(i)&&(w(s,!0),ne().then(()=>{w(t,ve(document.title||"untitled page"))}))});return w(i,!0),b});const d=O(()=>e.constructors[1]);var u=Re(),y=k(u);q(y,()=>e.constructors[1],b=>{var c=T();const m=O(()=>e.constructors[0]);var _=k(c);D(_,()=>v(m),(E,S)=>{B(S(E,{get data(){return a()},get form(){return e.form},children:(h,we)=>{var I=T(),W=k(I);D(W,()=>v(d),(G,H)=>{B(H(G,{get data(){return o()},get form(){return e.form}}),R=>r()[1]=R,()=>{var R;return(R=r())==null?void 0:R[1]})}),P(h,I)},$$slots:{default:!0}}),h=>r()[0]=h,()=>{var h;return(h=r())==null?void 0:h[0]})}),P(b,c)},b=>{var c=T();const m=O(()=>e.constructors[0]);var _=k(c);D(_,()=>v(m),(E,S)=>{B(S(E,{get data(){return a()},get form(){return e.form}}),h=>r()[0]=h,()=>{var h;return(h=r())==null?void 0:h[0]})}),P(b,c)});var x=oe(y,2);q(x,()=>v(i),b=>{var c=Pe(),m=ce(c);q(m,()=>v(s),_=>{var E=he();ie(()=>me(E,v(t))),P(_,E)}),le(c),P(b,c)}),P(n,u),ae()}const Te=ge(ke),je=[()=>F(()=>import("../nodes/0.NQgD_XHH.js"),__vite__mapDeps([0,1,2,3,4,5]),import.meta.url),()=>F(()=>import("../nodes/1.BIAsjNCt.js"),__vite__mapDeps([6,1,2,3,7,8,9,10]),import.meta.url),()=>F(()=>import("../nodes/2.nEq3Hs-V.js"),__vite__mapDeps([11,1,2,3,7,12,8,10,4,13]),import.meta.url)],qe=[],Be={"/":[2]},De={handleError:({error:n})=>{console.error(n)},reroute:()=>{}};export{Be as dictionary,De as hooks,Oe as matchers,je as nodes,Te as root,qe as server_loads};