#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `T` reader - 7-bit counter (MSB to LSB) These bits contain the value of the watchdog counter, decremented every (4096 x 2WDGTB\\[1:0\\]) PCLK cycles. A reset is produced when it is decremented from 0x40 to 0x3F (T6 becomes cleared)."]
pub type T_R = crate::FieldReader;
#[doc = "Field `T` writer - 7-bit counter (MSB to LSB) These bits contain the value of the watchdog counter, decremented every (4096 x 2WDGTB\\[1:0\\]) PCLK cycles. A reset is produced when it is decremented from 0x40 to 0x3F (T6 becomes cleared)."]
pub type T_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `WDGA` reader - Activation bit This bit is set by software and only cleared by hardware after a reset. When WDGA = 1, the watchdog can generate a reset."]
pub type WDGA_R = crate::BitReader;
#[doc = "Field `WDGA` writer - Activation bit This bit is set by software and only cleared by hardware after a reset. When WDGA = 1, the watchdog can generate a reset."]
pub type WDGA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - 7-bit counter (MSB to LSB) These bits contain the value of the watchdog counter, decremented every (4096 x 2WDGTB\\[1:0\\]) PCLK cycles. A reset is produced when it is decremented from 0x40 to 0x3F (T6 becomes cleared)."]
    #[inline(always)]
    pub fn t(&self) -> T_R {
        T_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Activation bit This bit is set by software and only cleared by hardware after a reset. When WDGA = 1, the watchdog can generate a reset."]
    #[inline(always)]
    pub fn wdga(&self) -> WDGA_R {
        WDGA_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - 7-bit counter (MSB to LSB) These bits contain the value of the watchdog counter, decremented every (4096 x 2WDGTB\\[1:0\\]) PCLK cycles. A reset is produced when it is decremented from 0x40 to 0x3F (T6 becomes cleared)."]
    #[inline(always)]
    #[must_use]
    pub fn t(&mut self) -> T_W<CRrs> {
        T_W::new(self, 0)
    }
    #[doc = "Bit 7 - Activation bit This bit is set by software and only cleared by hardware after a reset. When WDGA = 1, the watchdog can generate a reset."]
    #[inline(always)]
    #[must_use]
    pub fn wdga(&mut self) -> WDGA_W<CRrs> {
        WDGA_W::new(self, 7)
    }
}
#[doc = "WWDG control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0x7f"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x7f;
}
