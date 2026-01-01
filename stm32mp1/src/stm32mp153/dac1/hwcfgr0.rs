///Register `HWCFGR0` reader
pub type R = crate::R<HWCFGR0rs>;
///Field `DUAL` reader - DUAL
pub type DUAL_R = crate::FieldReader;
///Field `LFSR` reader - LFSR
pub type LFSR_R = crate::FieldReader;
///Field `TRIANGLE` reader - TRIANGLE
pub type TRIANGLE_R = crate::FieldReader;
///Field `SAMPLE` reader - SAMPLE
pub type SAMPLE_R = crate::FieldReader;
///Field `OR_CFG` reader - OR_CFG
pub type OR_CFG_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - DUAL
    #[inline(always)]
    pub fn dual(&self) -> DUAL_R {
        DUAL_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - LFSR
    #[inline(always)]
    pub fn lfsr(&self) -> LFSR_R {
        LFSR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - TRIANGLE
    #[inline(always)]
    pub fn triangle(&self) -> TRIANGLE_R {
        TRIANGLE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - SAMPLE
    #[inline(always)]
    pub fn sample(&self) -> SAMPLE_R {
        SAMPLE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:23 - OR_CFG
    #[inline(always)]
    pub fn or_cfg(&self) -> OR_CFG_R {
        OR_CFG_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR0")
            .field("dual", &self.dual())
            .field("lfsr", &self.lfsr())
            .field("triangle", &self.triangle())
            .field("sample", &self.sample())
            .field("or_cfg", &self.or_cfg())
            .finish()
    }
}
/**DAC IP hardware configuration register

You can [`read`](crate::Reg::read) this register and get [`hwcfgr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DAC1:HWCFGR0)*/
pub struct HWCFGR0rs;
impl crate::RegisterSpec for HWCFGR0rs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr0::R`](R) reader structure
impl crate::Readable for HWCFGR0rs {}
///`reset()` method sets HWCFGR0 to value 0x1111
impl crate::Resettable for HWCFGR0rs {
    const RESET_VALUE: u32 = 0x1111;
}
