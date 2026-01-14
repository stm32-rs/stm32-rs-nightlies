///Register `CFR` reader
pub type R = crate::R<CFRrs>;
///Register `CFR` writer
pub type W = crate::W<CFRrs>;
///Field `W` reader - W
pub type W_R = crate::FieldReader;
///Field `W` writer - W
pub type W_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `EWI` reader - EWI
pub type EWI_R = crate::BitReader;
///Field `EWI` writer - EWI
pub type EWI_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WDGTB` reader - WDGTB
pub type WDGTB_R = crate::FieldReader;
///Field `WDGTB` writer - WDGTB
pub type WDGTB_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:6 - W
    #[inline(always)]
    pub fn w(&self) -> W_R {
        W_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 9 - EWI
    #[inline(always)]
    pub fn ewi(&self) -> EWI_R {
        EWI_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 11:13 - WDGTB
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
    ///Bits 0:6 - W
    #[inline(always)]
    pub fn w(&mut self) -> W_W<'_, CFRrs> {
        W_W::new(self, 0)
    }
    ///Bit 9 - EWI
    #[inline(always)]
    pub fn ewi(&mut self) -> EWI_W<'_, CFRrs> {
        EWI_W::new(self, 9)
    }
    ///Bits 11:13 - WDGTB
    #[inline(always)]
    pub fn wdgtb(&mut self) -> WDGTB_W<'_, CFRrs> {
        WDGTB_W::new(self, 11)
    }
}
/**Configuration register

You can [`read`](crate::Reg::read) this register and get [`cfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#WWDG1:CFR)*/
pub struct CFRrs;
impl crate::RegisterSpec for CFRrs {
    type Ux = u16;
}
///`read()` method returns [`cfr::R`](R) reader structure
impl crate::Readable for CFRrs {}
///`write(|w| ..)` method takes [`cfr::W`](W) writer structure
impl crate::Writable for CFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFR to value 0x7f
impl crate::Resettable for CFRrs {
    const RESET_VALUE: u16 = 0x7f;
}
