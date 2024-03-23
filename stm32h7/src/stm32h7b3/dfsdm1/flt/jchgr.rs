#[doc = "Register `JCHGR` reader"]
pub type R = crate::R<JCHGRrs>;
#[doc = "Register `JCHGR` writer"]
pub type W = crate::W<JCHGRrs>;
#[doc = "Field `JCHG` reader - Injected channel group selection JCHG\\[y\\]=0: channel y is not part of the injected group JCHG\\[y\\]=1: channel y is part of the injected group If JSCAN=1, each of the selected channels is converted, one after another. The lowest channel (channel 0, if selected) is converted first and the sequence ends at the highest selected channel. If JSCAN=0, then only one channel is converted from the selected channels, and the channel selection is moved to the next channel. Writing JCHG, if JSCAN=0, resets the channel selection to the lowest selected channel. At least one channel must always be selected for the injected group. Writes causing all JCHG bits to be zero are ignored."]
pub type JCHG_R = crate::FieldReader;
#[doc = "Field `JCHG` writer - Injected channel group selection JCHG\\[y\\]=0: channel y is not part of the injected group JCHG\\[y\\]=1: channel y is part of the injected group If JSCAN=1, each of the selected channels is converted, one after another. The lowest channel (channel 0, if selected) is converted first and the sequence ends at the highest selected channel. If JSCAN=0, then only one channel is converted from the selected channels, and the channel selection is moved to the next channel. Writing JCHG, if JSCAN=0, resets the channel selection to the lowest selected channel. At least one channel must always be selected for the injected group. Writes causing all JCHG bits to be zero are ignored."]
pub type JCHG_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Injected channel group selection JCHG\\[y\\]=0: channel y is not part of the injected group JCHG\\[y\\]=1: channel y is part of the injected group If JSCAN=1, each of the selected channels is converted, one after another. The lowest channel (channel 0, if selected) is converted first and the sequence ends at the highest selected channel. If JSCAN=0, then only one channel is converted from the selected channels, and the channel selection is moved to the next channel. Writing JCHG, if JSCAN=0, resets the channel selection to the lowest selected channel. At least one channel must always be selected for the injected group. Writes causing all JCHG bits to be zero are ignored."]
    #[inline(always)]
    pub fn jchg(&self) -> JCHG_R {
        JCHG_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Injected channel group selection JCHG\\[y\\]=0: channel y is not part of the injected group JCHG\\[y\\]=1: channel y is part of the injected group If JSCAN=1, each of the selected channels is converted, one after another. The lowest channel (channel 0, if selected) is converted first and the sequence ends at the highest selected channel. If JSCAN=0, then only one channel is converted from the selected channels, and the channel selection is moved to the next channel. Writing JCHG, if JSCAN=0, resets the channel selection to the lowest selected channel. At least one channel must always be selected for the injected group. Writes causing all JCHG bits to be zero are ignored."]
    #[inline(always)]
    #[must_use]
    pub fn jchg(&mut self) -> JCHG_W<JCHGRrs> {
        JCHG_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jchgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jchgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JCHGRrs;
impl crate::RegisterSpec for JCHGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jchgr::R`](R) reader structure"]
impl crate::Readable for JCHGRrs {}
#[doc = "`write(|w| ..)` method takes [`jchgr::W`](W) writer structure"]
impl crate::Writable for JCHGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets JCHGR to value 0x01"]
impl crate::Resettable for JCHGRrs {
    const RESET_VALUE: u32 = 0x01;
}
