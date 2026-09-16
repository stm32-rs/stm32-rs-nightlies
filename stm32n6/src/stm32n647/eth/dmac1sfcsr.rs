///Register `DMAC1SFCSR` reader
pub type R = crate::R<DMAC1SFCSRrs>;
///Register `DMAC1SFCSR` writer
pub type W = crate::W<DMAC1SFCSRrs>;
///Field `ESC` reader - Enable Slot Comparison
pub type ESC_R = crate::BitReader;
///Field `ESC` writer - Enable Slot Comparison
pub type ESC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ASC` reader - Advance Slot Check
pub type ASC_R = crate::BitReader;
///Field `ASC` writer - Advance Slot Check
pub type ASC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SIV` reader - Slot Interval Value
pub type SIV_R = crate::FieldReader<u16>;
///Field `SIV` writer - Slot Interval Value
pub type SIV_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `RSN` reader - Reference Slot Number
pub type RSN_R = crate::FieldReader;
impl R {
    ///Bit 0 - Enable Slot Comparison
    #[inline(always)]
    pub fn esc(&self) -> ESC_R {
        ESC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Advance Slot Check
    #[inline(always)]
    pub fn asc(&self) -> ASC_R {
        ASC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 4:15 - Slot Interval Value
    #[inline(always)]
    pub fn siv(&self) -> SIV_R {
        SIV_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    ///Bits 16:19 - Reference Slot Number
    #[inline(always)]
    pub fn rsn(&self) -> RSN_R {
        RSN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAC1SFCSR")
            .field("esc", &self.esc())
            .field("asc", &self.asc())
            .field("siv", &self.siv())
            .field("rsn", &self.rsn())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable Slot Comparison
    #[inline(always)]
    pub fn esc(&mut self) -> ESC_W<'_, DMAC1SFCSRrs> {
        ESC_W::new(self, 0)
    }
    ///Bit 1 - Advance Slot Check
    #[inline(always)]
    pub fn asc(&mut self) -> ASC_W<'_, DMAC1SFCSRrs> {
        ASC_W::new(self, 1)
    }
    ///Bits 4:15 - Slot Interval Value
    #[inline(always)]
    pub fn siv(&mut self) -> SIV_W<'_, DMAC1SFCSRrs> {
        SIV_W::new(self, 4)
    }
}
/**Channel 1 slot function control status register

You can [`read`](crate::Reg::read) this register and get [`dmac1sfcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1sfcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#ETH:DMAC1SFCSR)*/
pub struct DMAC1SFCSRrs;
impl crate::RegisterSpec for DMAC1SFCSRrs {
    type Ux = u32;
}
///`read()` method returns [`dmac1sfcsr::R`](R) reader structure
impl crate::Readable for DMAC1SFCSRrs {}
///`write(|w| ..)` method takes [`dmac1sfcsr::W`](W) writer structure
impl crate::Writable for DMAC1SFCSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAC1SFCSR to value 0x07c0
impl crate::Resettable for DMAC1SFCSRrs {
    const RESET_VALUE: u32 = 0x07c0;
}
