///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `TES` reader - tearing source
pub type TES_R = crate::FieldReader;
///Field `TES` writer - tearing source
pub type TES_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TEPOL` reader - tearing--effect polarity
pub type TEPOL_R = crate::BitReader;
///Field `TEPOL` writer - tearing--effect polarity
pub type TEPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYNCS` reader - synchronization source
pub type SYNCS_R = crate::FieldReader;
///Field `SYNCS` writer - synchronization source
pub type SYNCS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `FCCOE` reader - frame-clock calibration output enable
pub type FCCOE_R = crate::BitReader;
///Field `FCCOE` writer - frame-clock calibration output enable
pub type FCCOE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCCOE` reader - line-clock calibration output enable
pub type LCCOE_R = crate::BitReader;
///Field `LCCOE` writer - line-clock calibration output enable
pub type LCCOE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - tearing source
    #[inline(always)]
    pub fn tes(&self) -> TES_R {
        TES_R::new((self.bits & 3) as u8)
    }
    ///Bit 4 - tearing--effect polarity
    #[inline(always)]
    pub fn tepol(&self) -> TEPOL_R {
        TEPOL_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 8:9 - synchronization source
    #[inline(always)]
    pub fn syncs(&self) -> SYNCS_R {
        SYNCS_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 16 - frame-clock calibration output enable
    #[inline(always)]
    pub fn fccoe(&self) -> FCCOE_R {
        FCCOE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - line-clock calibration output enable
    #[inline(always)]
    pub fn lccoe(&self) -> LCCOE_R {
        LCCOE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("tes", &self.tes())
            .field("tepol", &self.tepol())
            .field("syncs", &self.syncs())
            .field("fccoe", &self.fccoe())
            .field("lccoe", &self.lccoe())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - tearing source
    #[inline(always)]
    pub fn tes(&mut self) -> TES_W<'_, CRrs> {
        TES_W::new(self, 0)
    }
    ///Bit 4 - tearing--effect polarity
    #[inline(always)]
    pub fn tepol(&mut self) -> TEPOL_W<'_, CRrs> {
        TEPOL_W::new(self, 4)
    }
    ///Bits 8:9 - synchronization source
    #[inline(always)]
    pub fn syncs(&mut self) -> SYNCS_W<'_, CRrs> {
        SYNCS_W::new(self, 8)
    }
    ///Bit 16 - frame-clock calibration output enable
    #[inline(always)]
    pub fn fccoe(&mut self) -> FCCOE_W<'_, CRrs> {
        FCCOE_W::new(self, 16)
    }
    ///Bit 17 - line-clock calibration output enable
    #[inline(always)]
    pub fn lccoe(&mut self) -> LCCOE_W<'_, CRrs> {
        LCCOE_W::new(self, 17)
    }
}
/**GFXTIM configuration register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#GFXTIM:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
