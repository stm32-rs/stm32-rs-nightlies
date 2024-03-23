#[doc = "Register `ETH_MACSSIR` reader"]
pub type R = crate::R<ETH_MACSSIRrs>;
#[doc = "Register `ETH_MACSSIR` writer"]
pub type W = crate::W<ETH_MACSSIRrs>;
#[doc = "Field `SNSINC` reader - SNSINC"]
pub type SNSINC_R = crate::FieldReader;
#[doc = "Field `SNSINC` writer - SNSINC"]
pub type SNSINC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SSINC` reader - SSINC"]
pub type SSINC_R = crate::FieldReader;
#[doc = "Field `SSINC` writer - SSINC"]
pub type SSINC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 8:15 - SNSINC"]
    #[inline(always)]
    pub fn snsinc(&self) -> SNSINC_R {
        SNSINC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SSINC"]
    #[inline(always)]
    pub fn ssinc(&self) -> SSINC_R {
        SSINC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - SNSINC"]
    #[inline(always)]
    #[must_use]
    pub fn snsinc(&mut self) -> SNSINC_W<ETH_MACSSIRrs> {
        SNSINC_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - SSINC"]
    #[inline(always)]
    #[must_use]
    pub fn ssinc(&mut self) -> SSINC_W<ETH_MACSSIRrs> {
        SSINC_W::new(self, 16)
    }
}
#[doc = "The Sub-second Increment register is present only when the IEEE 1588 timestamp feature is selected without an external timestamp input. In Coarse Update mode \\[Bit 1 in ETH_MACTSCR register, the value in this register is added to the system time every clock cycle of HCLK. In Fine Update mode, the value in this register is added to the system time whenever the Accumulator gets an overflow.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macssir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macssir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACSSIRrs;
impl crate::RegisterSpec for ETH_MACSSIRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_macssir::R`](R) reader structure"]
impl crate::Readable for ETH_MACSSIRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_macssir::W`](W) writer structure"]
impl crate::Writable for ETH_MACSSIRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MACSSIR to value 0"]
impl crate::Resettable for ETH_MACSSIRrs {
    const RESET_VALUE: u32 = 0;
}
