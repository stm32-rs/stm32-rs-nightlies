///Register `WWDG_CFR` reader
pub type R = crate::R<WWDG_CFRrs>;
///Register `WWDG_CFR` writer
pub type W = crate::W<WWDG_CFRrs>;
///Field `W` reader - 7-bit window value These bits contain the window value to be compared with the down-counter.
pub type W_R = crate::FieldReader;
///Field `W` writer - 7-bit window value These bits contain the window value to be compared with the down-counter.
pub type W_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `EWI` reader - Early wake-up interrupt enable Set by software and cleared by hardware after a reset. When set, an interrupt occurs whenever the counter reaches the value 0x40.
pub type EWI_R = crate::BitReader;
///Field `EWI` writer - Early wake-up interrupt enable Set by software and cleared by hardware after a reset. When set, an interrupt occurs whenever the counter reaches the value 0x40.
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
    ///Bit 9 - Early wake-up interrupt enable Set by software and cleared by hardware after a reset. When set, an interrupt occurs whenever the counter reaches the value 0x40.
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
        f.debug_struct("WWDG_CFR")
            .field("w", &self.w())
            .field("ewi", &self.ewi())
            .field("wdgtb", &self.wdgtb())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - 7-bit window value These bits contain the window value to be compared with the down-counter.
    #[inline(always)]
    pub fn w(&mut self) -> W_W<WWDG_CFRrs> {
        W_W::new(self, 0)
    }
    ///Bit 9 - Early wake-up interrupt enable Set by software and cleared by hardware after a reset. When set, an interrupt occurs whenever the counter reaches the value 0x40.
    #[inline(always)]
    pub fn ewi(&mut self) -> EWI_W<WWDG_CFRrs> {
        EWI_W::new(self, 9)
    }
    ///Bits 11:13 - Timer base The timebase of the prescaler can be modified as follows:
    #[inline(always)]
    pub fn wdgtb(&mut self) -> WDGTB_W<WWDG_CFRrs> {
        WDGTB_W::new(self, 11)
    }
}
/**WWDG configuration register

You can [`read`](crate::Reg::read) this register and get [`wwdg_cfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wwdg_cfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#WWDG:WWDG_CFR)*/
pub struct WWDG_CFRrs;
impl crate::RegisterSpec for WWDG_CFRrs {
    type Ux = u32;
}
///`read()` method returns [`wwdg_cfr::R`](R) reader structure
impl crate::Readable for WWDG_CFRrs {}
///`write(|w| ..)` method takes [`wwdg_cfr::W`](W) writer structure
impl crate::Writable for WWDG_CFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets WWDG_CFR to value 0x7f
impl crate::Resettable for WWDG_CFRrs {
    const RESET_VALUE: u32 = 0x7f;
}