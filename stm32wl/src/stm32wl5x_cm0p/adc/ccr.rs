///Register `CCR` reader
pub type R = crate::R<CCRrs>;
///Register `CCR` writer
pub type W = crate::W<CCRrs>;
///Field `PRESC0` reader - PRESC0
pub type PRESC0_R = crate::BitReader;
///Field `PRESC0` writer - PRESC0
pub type PRESC0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRESC1` reader - PRESC1
pub type PRESC1_R = crate::BitReader;
///Field `PRESC1` writer - PRESC1
pub type PRESC1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRESC2` reader - PRESC2
pub type PRESC2_R = crate::BitReader;
///Field `PRESC2` writer - PRESC2
pub type PRESC2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRESC3` reader - PRESC3
pub type PRESC3_R = crate::BitReader;
///Field `PRESC3` writer - PRESC3
pub type PRESC3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VREFEN` reader - VREFEN
pub type VREFEN_R = crate::BitReader;
///Field `VREFEN` writer - VREFEN
pub type VREFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSEN` reader - TSEN
pub type TSEN_R = crate::BitReader;
///Field `TSEN` writer - TSEN
pub type TSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VBATEN` reader - VBATEN
pub type VBATEN_R = crate::BitReader;
///Field `VBATEN` writer - VBATEN
pub type VBATEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 18 - PRESC0
    #[inline(always)]
    pub fn presc0(&self) -> PRESC0_R {
        PRESC0_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - PRESC1
    #[inline(always)]
    pub fn presc1(&self) -> PRESC1_R {
        PRESC1_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - PRESC2
    #[inline(always)]
    pub fn presc2(&self) -> PRESC2_R {
        PRESC2_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - PRESC3
    #[inline(always)]
    pub fn presc3(&self) -> PRESC3_R {
        PRESC3_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - VREFEN
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - TSEN
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - VBATEN
    #[inline(always)]
    pub fn vbaten(&self) -> VBATEN_R {
        VBATEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR")
            .field("presc0", &self.presc0())
            .field("presc1", &self.presc1())
            .field("presc2", &self.presc2())
            .field("presc3", &self.presc3())
            .field("vrefen", &self.vrefen())
            .field("tsen", &self.tsen())
            .field("vbaten", &self.vbaten())
            .finish()
    }
}
impl W {
    ///Bit 18 - PRESC0
    #[inline(always)]
    pub fn presc0(&mut self) -> PRESC0_W<CCRrs> {
        PRESC0_W::new(self, 18)
    }
    ///Bit 19 - PRESC1
    #[inline(always)]
    pub fn presc1(&mut self) -> PRESC1_W<CCRrs> {
        PRESC1_W::new(self, 19)
    }
    ///Bit 20 - PRESC2
    #[inline(always)]
    pub fn presc2(&mut self) -> PRESC2_W<CCRrs> {
        PRESC2_W::new(self, 20)
    }
    ///Bit 21 - PRESC3
    #[inline(always)]
    pub fn presc3(&mut self) -> PRESC3_W<CCRrs> {
        PRESC3_W::new(self, 21)
    }
    ///Bit 22 - VREFEN
    #[inline(always)]
    pub fn vrefen(&mut self) -> VREFEN_W<CCRrs> {
        VREFEN_W::new(self, 22)
    }
    ///Bit 23 - TSEN
    #[inline(always)]
    pub fn tsen(&mut self) -> TSEN_W<CCRrs> {
        TSEN_W::new(self, 23)
    }
    ///Bit 24 - VBATEN
    #[inline(always)]
    pub fn vbaten(&mut self) -> VBATEN_W<CCRrs> {
        VBATEN_W::new(self, 24)
    }
}
/**ADC common configuration register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#ADC:CCR)*/
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u32;
}
///`read()` method returns [`ccr::R`](R) reader structure
impl crate::Readable for CCRrs {}
///`write(|w| ..)` method takes [`ccr::W`](W) writer structure
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CCR to value 0
impl crate::Resettable for CCRrs {
    const RESET_VALUE: u32 = 0;
}
