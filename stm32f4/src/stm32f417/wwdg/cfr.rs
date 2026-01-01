///Register `CFR` reader
pub type R = crate::R<CFRrs>;
///Register `CFR` writer
pub type W = crate::W<CFRrs>;
///Field `W` reader - 7-bit window value
pub type W_R = crate::FieldReader;
///Field `W` writer - 7-bit window value
pub type W_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `WDGTB0` reader - Timer base
pub type WDGTB0_R = crate::BitReader;
///Field `WDGTB0` writer - Timer base
pub type WDGTB0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WDGTB1` reader - Timer base
pub type WDGTB1_R = crate::BitReader;
///Field `WDGTB1` writer - Timer base
pub type WDGTB1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWI` reader - Early wakeup interrupt
pub type EWI_R = crate::BitReader;
///Field `EWI` writer - Early wakeup interrupt
pub type EWI_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:6 - 7-bit window value
    #[inline(always)]
    pub fn w(&self) -> W_R {
        W_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 7 - Timer base
    #[inline(always)]
    pub fn wdgtb0(&self) -> WDGTB0_R {
        WDGTB0_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Timer base
    #[inline(always)]
    pub fn wdgtb1(&self) -> WDGTB1_R {
        WDGTB1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Early wakeup interrupt
    #[inline(always)]
    pub fn ewi(&self) -> EWI_R {
        EWI_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFR")
            .field("ewi", &self.ewi())
            .field("wdgtb1", &self.wdgtb1())
            .field("wdgtb0", &self.wdgtb0())
            .field("w", &self.w())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - 7-bit window value
    #[inline(always)]
    pub fn w(&mut self) -> W_W<'_, CFRrs> {
        W_W::new(self, 0)
    }
    ///Bit 7 - Timer base
    #[inline(always)]
    pub fn wdgtb0(&mut self) -> WDGTB0_W<'_, CFRrs> {
        WDGTB0_W::new(self, 7)
    }
    ///Bit 8 - Timer base
    #[inline(always)]
    pub fn wdgtb1(&mut self) -> WDGTB1_W<'_, CFRrs> {
        WDGTB1_W::new(self, 8)
    }
    ///Bit 9 - Early wakeup interrupt
    #[inline(always)]
    pub fn ewi(&mut self) -> EWI_W<'_, CFRrs> {
        EWI_W::new(self, 9)
    }
}
/**Configuration register

You can [`read`](crate::Reg::read) this register and get [`cfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#WWDG:CFR)*/
pub struct CFRrs;
impl crate::RegisterSpec for CFRrs {
    type Ux = u32;
}
///`read()` method returns [`cfr::R`](R) reader structure
impl crate::Readable for CFRrs {}
///`write(|w| ..)` method takes [`cfr::W`](W) writer structure
impl crate::Writable for CFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFR to value 0x7f
impl crate::Resettable for CFRrs {
    const RESET_VALUE: u32 = 0x7f;
}
