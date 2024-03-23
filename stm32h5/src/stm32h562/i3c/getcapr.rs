#[doc = "Register `GETCAPR` reader"]
pub type R = crate::R<GETCAPRrs>;
#[doc = "Register `GETCAPR` writer"]
pub type W = crate::W<GETCAPRrs>;
#[doc = "Field `CAPPEND` reader - IBI MDB support for pending read notification This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates the support (or not) of the pending read notification via the IBI MDB\\[7:0\\]
value. This bit is used to return the GETCAP3 byte in response to the GETCAPS CCC format 1."]
pub type CAPPEND_R = crate::BitReader;
#[doc = "Field `CAPPEND` writer - IBI MDB support for pending read notification This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates the support (or not) of the pending read notification via the IBI MDB\\[7:0\\]
value. This bit is used to return the GETCAP3 byte in response to the GETCAPS CCC format 1."]
pub type CAPPEND_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 14 - IBI MDB support for pending read notification This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates the support (or not) of the pending read notification via the IBI MDB\\[7:0\\]
value. This bit is used to return the GETCAP3 byte in response to the GETCAPS CCC format 1."]
    #[inline(always)]
    pub fn cappend(&self) -> CAPPEND_R {
        CAPPEND_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - IBI MDB support for pending read notification This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates the support (or not) of the pending read notification via the IBI MDB\\[7:0\\]
value. This bit is used to return the GETCAP3 byte in response to the GETCAPS CCC format 1."]
    #[inline(always)]
    #[must_use]
    pub fn cappend(&mut self) -> CAPPEND_W<GETCAPRrs> {
        CAPPEND_W::new(self, 14)
    }
}
#[doc = "I3C get capability register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`getcapr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`getcapr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GETCAPRrs;
impl crate::RegisterSpec for GETCAPRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`getcapr::R`](R) reader structure"]
impl crate::Readable for GETCAPRrs {}
#[doc = "`write(|w| ..)` method takes [`getcapr::W`](W) writer structure"]
impl crate::Writable for GETCAPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GETCAPR to value 0"]
impl crate::Resettable for GETCAPRrs {
    const RESET_VALUE: u32 = 0;
}
