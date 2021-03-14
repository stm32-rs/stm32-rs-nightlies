#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OctoSPI IO Manager Control Register"]
    pub cr: CR,
    #[doc = "0x04 - OctoSPI IO Manager Port 1 configuration register"]
    pub p1cr: P1CR,
    #[doc = "0x08 - OctoSPI IO Manager Port 2 configuration register"]
    pub p2cr: P2CR,
}
#[doc = "OctoSPI IO Manager Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "OctoSPI IO Manager Control Register"]
pub mod cr;
#[doc = "OctoSPI IO Manager Port 1 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p1cr](p1cr) module"]
pub type P1CR = crate::Reg<u32, _P1CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P1CR;
#[doc = "`read()` method returns [p1cr::R](p1cr::R) reader structure"]
impl crate::Readable for P1CR {}
#[doc = "`write(|w| ..)` method takes [p1cr::W](p1cr::W) writer structure"]
impl crate::Writable for P1CR {}
#[doc = "OctoSPI IO Manager Port 1 configuration register"]
pub mod p1cr;
#[doc = "OctoSPI IO Manager Port 2 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p2cr](p2cr) module"]
pub type P2CR = crate::Reg<u32, _P2CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P2CR;
#[doc = "`read()` method returns [p2cr::R](p2cr::R) reader structure"]
impl crate::Readable for P2CR {}
#[doc = "`write(|w| ..)` method takes [p2cr::W](p2cr::W) writer structure"]
impl crate::Writable for P2CR {}
#[doc = "OctoSPI IO Manager Port 2 configuration register"]
pub mod p2cr;
