#[doc = "Register `ETH_MACSTNUR` reader"]
pub type R = crate::R<ETH_MACSTNURrs>;
#[doc = "Register `ETH_MACSTNUR` writer"]
pub type W = crate::W<ETH_MACSTNURrs>;
#[doc = "Field `TSSS` reader - TSSS"]
pub type TSSS_R = crate::FieldReader<u32>;
#[doc = "Field `TSSS` writer - TSSS"]
pub type TSSS_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `ADDSUB` reader - ADDSUB"]
pub type ADDSUB_R = crate::BitReader;
#[doc = "Field `ADDSUB` writer - ADDSUB"]
pub type ADDSUB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30 - TSSS"]
    #[inline(always)]
    pub fn tsss(&self) -> TSSS_R {
        TSSS_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - ADDSUB"]
    #[inline(always)]
    pub fn addsub(&self) -> ADDSUB_R {
        ADDSUB_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - TSSS"]
    #[inline(always)]
    #[must_use]
    pub fn tsss(&mut self) -> TSSS_W<ETH_MACSTNURrs> {
        TSSS_W::new(self, 0)
    }
    #[doc = "Bit 31 - ADDSUB"]
    #[inline(always)]
    #[must_use]
    pub fn addsub(&mut self) -> ADDSUB_W<ETH_MACSTNURrs> {
        ADDSUB_W::new(self, 31)
    }
}
#[doc = "This register is present only when the IEEE 1588 timestamp feature is selected without external timestamp input.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macstnur::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macstnur::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACSTNURrs;
impl crate::RegisterSpec for ETH_MACSTNURrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_macstnur::R`](R) reader structure"]
impl crate::Readable for ETH_MACSTNURrs {}
#[doc = "`write(|w| ..)` method takes [`eth_macstnur::W`](W) writer structure"]
impl crate::Writable for ETH_MACSTNURrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MACSTNUR to value 0"]
impl crate::Resettable for ETH_MACSTNURrs {
    const RESET_VALUE: u32 = 0;
}
