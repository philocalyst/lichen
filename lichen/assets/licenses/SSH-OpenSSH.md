{{#if placeholder0}}{{placeholder0}}{{else}}Copyright (c) 1995 Tatu Ylonen <ylo@cs.hut.fi>, Espoo, Finland
                               All rights reserved{{/if}}

As far as I am concerned, the code I have written for this software can be used freely for any purpose. Any derived versions of this software must be clearly marked as such, and if the derived work is incompatible with the protocol description in the RFC file, it must be called by a name other than &quot;ssh&quot; or &quot;Secure Shell&quot;.

[Tatu continues]

However, I am not implying to give any licenses to any patents or copyrights held by third parties, and the software includes parts that are not under my direct control. As far as I know, all included source code is used in accordance with the relevant license agreements and can be used freely for any purpose (the GNU license being the most restrictive); see below for details.

[However, none of that term is relevant at this point in time. All of these restrictively licenced software components which he talks about have been removed from OpenSSH, i.e.,

* {{#if placeholder1}}{{placeholder1}}{{else}}-{{/if}} RSA is no longer included, found in the OpenSSL library
* {{#if placeholder2}}{{placeholder2}}{{else}}-{{/if}} IDEA is no longer included, its use is deprecated
* {{#if placeholder3}}{{placeholder3}}{{else}}-{{/if}} DES is now external, in the OpenSSL library
* {{#if placeholder4}}{{placeholder4}}{{else}}-{{/if}} GMP is no longer used, and instead we call BN code from OpenSSL
* {{#if placeholder5}}{{placeholder5}}{{else}}-{{/if}} Zlib is now external, in a library
* {{#if placeholder6}}{{placeholder6}}{{else}}-{{/if}} The make-ssh-known-hosts script is no longer included
* {{#if placeholder7}}{{placeholder7}}{{else}}-{{/if}} TSS has been removed
* {{#if placeholder8}}{{placeholder8}}{{else}}-{{/if}} MD5 is now external, in the OpenSSL library
* {{#if placeholder9}}{{placeholder9}}{{else}}-{{/if}} RC4 support has been replaced with ARC4 support from OpenSSL
* {{#if placeholder10}}{{placeholder10}}{{else}}-{{/if}} Blowfish is now external, in the OpenSSL library

[The licence continues]

Note that any information and cryptographic algorithms used in this software are publicly available on the Internet and at any major bookstore, scientific library, and patent office worldwide. More information can be found e.g. at &quot;http://www.cs.hut.fi/crypto&quot;.

The legal status of this program is some combination of all these permissions and restrictions. Use only at your own responsibility. You will be responsible for any legal consequences yourself; I am not making any claims whether possessing or using this is legal or not in your country, and I am not taking any responsibility on your behalf.

NO WARRANTY

BECAUSE THE PROGRAM IS LICENSED FREE OF CHARGE, THERE IS NO WARRANTY FOR THE PROGRAM, TO THE EXTENT PERMITTED BY APPLICABLE LAW. EXCEPT WHEN OTHERWISE STATED IN WRITING THE COPYRIGHT HOLDERS AND/OR OTHER PARTIES PROVIDE THE PROGRAM &quot;AS IS&quot; WITHOUT WARRANTY OF ANY KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE. THE ENTIRE RISK AS TO THE QUALITY AND PERFORMANCE OF THE PROGRAM IS WITH YOU. SHOULD THE PROGRAM PROVE DEFECTIVE, YOU ASSUME THE COST OF ALL NECESSARY SERVICING, REPAIR OR CORRECTION.

IN NO EVENT UNLESS REQUIRED BY APPLICABLE LAW OR AGREED TO IN WRITING WILL ANY COPYRIGHT HOLDER, OR ANY OTHER PARTY WHO MAY MODIFY AND/OR REDISTRIBUTE THE PROGRAM AS PERMITTED ABOVE, BE LIABLE TO YOU FOR DAMAGES, INCLUDING ANY GENERAL, SPECIAL, INCIDENTAL OR CONSEQUENTIAL DAMAGES ARISING OUT OF THE USE OR INABILITY TO USE THE PROGRAM (INCLUDING BUT NOT LIMITED TO LOSS OF DATA OR DATA BEING RENDERED INACCURATE OR LOSSES SUSTAINED BY YOU OR THIRD PARTIES OR A FAILURE OF THE PROGRAM TO OPERATE WITH ANY OTHER PROGRAMS), EVEN IF SUCH HOLDER OR OTHER PARTY HAS BEEN ADVISED OF THE POSSIBILITY OF SUCH DAMAGES.