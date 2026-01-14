///Register `TURCF` reader
pub type R = crate::R<TURCFrs>;
///Register `TURCF` writer
pub type W = crate::W<TURCFrs>;
///Field `NCL` reader - Numerator Configuration Low.
pub type NCL_R = crate::FieldReader<u16>;
///Field `NCL` writer - Numerator Configuration Low.
pub type NCL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `DC` reader - Denominator Configuration.
pub type DC_R = crate::FieldReader<u16>;
///Field `DC` writer - Denominator Configuration.
pub type DC_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
///Field `ELT` reader - Enable Local Time
pub type ELT_R = crate::BitReader;
///Field `ELT` writer - Enable Local Time
pub type ELT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - Numerator Configuration Low.
    #[inline(always)]
    pub fn ncl(&self) -> NCL_R {
        NCL_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:29 - Denominator Configuration.
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    ///Bit 31 - Enable Local Time
    #[inline(always)]
    pub fn elt(&self) -> ELT_R {
        ELT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TURCF")
            .field("ncl", &self.ncl())
            .field("dc", &self.dc())
            .field("elt", &self.elt())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Numerator Configuration Low.
    #[inline(always)]
    pub fn ncl(&mut self) -> NCL_W<'_, TURCFrs> {
        NCL_W::new(self, 0)
    }
    ///Bits 16:29 - Denominator Configuration.
    #[inline(always)]
    pub fn dc(&mut self) -> DC_W<'_, TURCFrs> {
        DC_W::new(self, 16)
    }
    ///Bit 31 - Enable Local Time
    #[inline(always)]
    pub fn elt(&mut self) -> ELT_W<'_, TURCFrs> {
        ELT_W::new(self, 31)
    }
}
/**FDCAN TUR Configuration Register

You can [`read`](crate::Reg::read) this register and get [`turcf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`turcf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#FDCAN1:TURCF)*/
pub struct TURCFrs;
impl crate::RegisterSpec for TURCFrs {
    type Ux = u32;
}
///`read()` method returns [`turcf::R`](R) reader structure
impl crate::Readable for TURCFrs {}
///`write(|w| ..)` method takes [`turcf::W`](W) writer structure
impl crate::Writable for TURCFrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TURCF to value 0
impl crate::Resettable for TURCFrs {}
