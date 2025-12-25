///Register `DFLT0CICR` reader
pub type R = crate::R<DFLT0CICRrs>;
///Register `DFLT0CICR` writer
pub type W = crate::W<DFLT0CICRrs>;
///Field `DATSRC` reader - Source data for the digital filter This field is set and cleared by software. 0x: Stream coming from the BSMX selected Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
pub type DATSRC_R = crate::FieldReader;
///Field `DATSRC` writer - Source data for the digital filter This field is set and cleared by software. 0x: Stream coming from the BSMX selected Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
pub type DATSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CICMOD` reader - Select the CIC order This field is set and cleared by software. It is used to select the order of the MCIC. others: reserved Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
pub type CICMOD_R = crate::FieldReader;
///Field `CICMOD` writer - Select the CIC order This field is set and cleared by software. It is used to select the order of the MCIC. others: reserved Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
pub type CICMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MCICD` reader - CIC decimation ratio selection This field is set and cleared by software.It is used to select the CIC decimation ratio. A decimation ratio smaller than two is not allowed. The decimation ratio is given by (CICDEC+1). ... Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
pub type MCICD_R = crate::FieldReader;
///Field `MCICD` writer - CIC decimation ratio selection This field is set and cleared by software.It is used to select the CIC decimation ratio. A decimation ratio smaller than two is not allowed. The decimation ratio is given by (CICDEC+1). ... Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
pub type MCICD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `MCICD8` reader - CIC decimation ratio selection This field is set and cleared by software.It is used to select the CIC decimation ratio. A decimation ratio smaller than two is not allowed. The decimation ratio is given by (CICDEC+1). ... Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
pub type MCICD8_R = crate::BitReader;
///Field `MCICD8` writer - CIC decimation ratio selection This field is set and cleared by software.It is used to select the CIC decimation ratio. A decimation ratio smaller than two is not allowed. The decimation ratio is given by (CICDEC+1). ... Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
pub type MCICD8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCALE` reader - Scaling factor selection This field is set and cleared by software. It is used to select the gain to be applied at CIC output (see Table 419 for details). If the application attempts to write a new gain value while the previous one is not yet applied, this new gain value is ignored. Reading back this field informs the application on the current gain value. ... ... others: Reserved
pub type SCALE_R = crate::FieldReader;
///Field `SCALE` writer - Scaling factor selection This field is set and cleared by software. It is used to select the gain to be applied at CIC output (see Table 419 for details). If the application attempts to write a new gain value while the previous one is not yet applied, this new gain value is ignored. Reading back this field informs the application on the current gain value. ... ... others: Reserved
pub type SCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:1 - Source data for the digital filter This field is set and cleared by software. 0x: Stream coming from the BSMX selected Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
    #[inline(always)]
    pub fn datsrc(&self) -> DATSRC_R {
        DATSRC_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:6 - Select the CIC order This field is set and cleared by software. It is used to select the order of the MCIC. others: reserved Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
    #[inline(always)]
    pub fn cicmod(&self) -> CICMOD_R {
        CICMOD_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:15 - CIC decimation ratio selection This field is set and cleared by software.It is used to select the CIC decimation ratio. A decimation ratio smaller than two is not allowed. The decimation ratio is given by (CICDEC+1). ... Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
    #[inline(always)]
    pub fn mcicd(&self) -> MCICD_R {
        MCICD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bit 16 - CIC decimation ratio selection This field is set and cleared by software.It is used to select the CIC decimation ratio. A decimation ratio smaller than two is not allowed. The decimation ratio is given by (CICDEC+1). ... Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
    #[inline(always)]
    pub fn mcicd8(&self) -> MCICD8_R {
        MCICD8_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 20:25 - Scaling factor selection This field is set and cleared by software. It is used to select the gain to be applied at CIC output (see Table 419 for details). If the application attempts to write a new gain value while the previous one is not yet applied, this new gain value is ignored. Reading back this field informs the application on the current gain value. ... ... others: Reserved
    #[inline(always)]
    pub fn scale(&self) -> SCALE_R {
        SCALE_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFLT0CICR")
            .field("datsrc", &self.datsrc())
            .field("cicmod", &self.cicmod())
            .field("mcicd", &self.mcicd())
            .field("mcicd8", &self.mcicd8())
            .field("scale", &self.scale())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Source data for the digital filter This field is set and cleared by software. 0x: Stream coming from the BSMX selected Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
    #[inline(always)]
    pub fn datsrc(&mut self) -> DATSRC_W<'_, DFLT0CICRrs> {
        DATSRC_W::new(self, 0)
    }
    ///Bits 4:6 - Select the CIC order This field is set and cleared by software. It is used to select the order of the MCIC. others: reserved Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
    #[inline(always)]
    pub fn cicmod(&mut self) -> CICMOD_W<'_, DFLT0CICRrs> {
        CICMOD_W::new(self, 4)
    }
    ///Bits 8:15 - CIC decimation ratio selection This field is set and cleared by software.It is used to select the CIC decimation ratio. A decimation ratio smaller than two is not allowed. The decimation ratio is given by (CICDEC+1). ... Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
    #[inline(always)]
    pub fn mcicd(&mut self) -> MCICD_W<'_, DFLT0CICRrs> {
        MCICD_W::new(self, 8)
    }
    ///Bit 16 - CIC decimation ratio selection This field is set and cleared by software.It is used to select the CIC decimation ratio. A decimation ratio smaller than two is not allowed. The decimation ratio is given by (CICDEC+1). ... Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
    #[inline(always)]
    pub fn mcicd8(&mut self) -> MCICD8_W<'_, DFLT0CICRrs> {
        MCICD8_W::new(self, 16)
    }
    ///Bits 20:25 - Scaling factor selection This field is set and cleared by software. It is used to select the gain to be applied at CIC output (see Table 419 for details). If the application attempts to write a new gain value while the previous one is not yet applied, this new gain value is ignored. Reading back this field informs the application on the current gain value. ... ... others: Reserved
    #[inline(always)]
    pub fn scale(&mut self) -> SCALE_W<'_, DFLT0CICRrs> {
        SCALE_W::new(self, 20)
    }
}
/**ADF digital filer configuration register 0

You can [`read`](crate::Reg::read) this register and get [`dflt0cicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt0cicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ADF:DFLT0CICR)*/
pub struct DFLT0CICRrs;
impl crate::RegisterSpec for DFLT0CICRrs {
    type Ux = u32;
}
///`read()` method returns [`dflt0cicr::R`](R) reader structure
impl crate::Readable for DFLT0CICRrs {}
///`write(|w| ..)` method takes [`dflt0cicr::W`](W) writer structure
impl crate::Writable for DFLT0CICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DFLT0CICR to value 0
impl crate::Resettable for DFLT0CICRrs {}
