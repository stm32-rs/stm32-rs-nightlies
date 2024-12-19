///Register `CFR` reader
pub type R = crate::R<CFRrs>;
///Register `CFR` writer
pub type W = crate::W<CFRrs>;
///Field `W` reader - 7-bit window value These bits contain the window value to be compared with the down-counter.
pub type W_R = crate::FieldReader;
///Field `W` writer - 7-bit window value These bits contain the window value to be compared with the down-counter.
pub type W_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `EWI` reader - Early wakeup interrupt When set, an interrupt occurs whenever the counter reaches the value 0x40. This interrupt is only cleared by hardware after a reset.
pub type EWI_R = crate::BitReader;
///Field `EWI` writer - Early wakeup interrupt When set, an interrupt occurs whenever the counter reaches the value 0x40. This interrupt is only cleared by hardware after a reset.
pub type EWI_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WDGTB` reader - Timer base The timebase of the prescaler can be modified as follows:
pub type WDGTB_R = crate::FieldReader;
///Field `WDGTB` writer - Timer base The timebase of the prescaler can be modified as follows:
pub type WDGTB_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:6 - 7-bit window value These bits contain the window value to be compared with the down-counter.
    #[inline(always)]
    pub fn w(&self) -> W_R {
        W_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 9 - Early wakeup interrupt When set, an interrupt occurs whenever the counter reaches the value 0x40. This interrupt is only cleared by hardware after a reset.
    #[inline(always)]
    pub fn ewi(&self) -> EWI_R {
        EWI_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 11:13 - Timer base The timebase of the prescaler can be modified as follows:
    #[inline(always)]
    pub fn wdgtb(&self) -> WDGTB_R {
        WDGTB_R::new(((self.bits >> 11) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFR")
            .field("w", &self.w())
            .field("ewi", &self.ewi())
            .field("wdgtb", &self.wdgtb())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - 7-bit window value These bits contain the window value to be compared with the down-counter.
    #[inline(always)]
    pub fn w(&mut self) -> W_W<CFRrs> {
        W_W::new(self, 0)
    }
    ///Bit 9 - Early wakeup interrupt When set, an interrupt occurs whenever the counter reaches the value 0x40. This interrupt is only cleared by hardware after a reset.
    #[inline(always)]
    pub fn ewi(&mut self) -> EWI_W<CFRrs> {
        EWI_W::new(self, 9)
    }
    ///Bits 11:13 - Timer base The timebase of the prescaler can be modified as follows:
    #[inline(always)]
    pub fn wdgtb(&mut self) -> WDGTB_W<CFRrs> {
        WDGTB_W::new(self, 11)
    }
}
/**WWDG configuration register

You can [`read`](crate::Reg::read) this register and get [`cfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#WWDG:CFR)*/
pub struct CFRrs;
impl crate::RegisterSpec for CFRrs {
    type Ux = u16;
}
///`read()` method returns [`cfr::R`](R) reader structure
impl crate::Readable for CFRrs {}
///`write(|w| ..)` method takes [`cfr::W`](W) writer structure
impl crate::Writable for CFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
///`reset()` method sets CFR to value 0x7f
impl crate::Resettable for CFRrs {
    const RESET_VALUE: u16 = 0x7f;
}
