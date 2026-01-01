///Register `IP_HWCFGR0` reader
pub type R = crate::R<IP_HWCFGR0rs>;
///Register `IP_HWCFGR0` writer
pub type W = crate::W<IP_HWCFGR0rs>;
///Field `DUAL` reader - Dual DAC capability
pub type DUAL_R = crate::FieldReader;
///Field `DUAL` writer - Dual DAC capability
pub type DUAL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `LFSR` reader - Pseudonoise wave generation capability
pub type LFSR_R = crate::FieldReader;
///Field `LFSR` writer - Pseudonoise wave generation capability
pub type LFSR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TRIANGLE` reader - Triangle wave generation capability
pub type TRIANGLE_R = crate::FieldReader;
///Field `TRIANGLE` writer - Triangle wave generation capability
pub type TRIANGLE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SAMPLE` reader - Sample and hold mode capability
pub type SAMPLE_R = crate::FieldReader;
///Field `SAMPLE` writer - Sample and hold mode capability
pub type SAMPLE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `OR_CFG` reader - option register bit width
pub type OR_CFG_R = crate::FieldReader;
///Field `OR_CFG` writer - option register bit width
pub type OR_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:3 - Dual DAC capability
    #[inline(always)]
    pub fn dual(&self) -> DUAL_R {
        DUAL_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Pseudonoise wave generation capability
    #[inline(always)]
    pub fn lfsr(&self) -> LFSR_R {
        LFSR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Triangle wave generation capability
    #[inline(always)]
    pub fn triangle(&self) -> TRIANGLE_R {
        TRIANGLE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Sample and hold mode capability
    #[inline(always)]
    pub fn sample(&self) -> SAMPLE_R {
        SAMPLE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:23 - option register bit width
    #[inline(always)]
    pub fn or_cfg(&self) -> OR_CFG_R {
        OR_CFG_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IP_HWCFGR0")
            .field("dual", &self.dual())
            .field("lfsr", &self.lfsr())
            .field("triangle", &self.triangle())
            .field("sample", &self.sample())
            .field("or_cfg", &self.or_cfg())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Dual DAC capability
    #[inline(always)]
    pub fn dual(&mut self) -> DUAL_W<'_, IP_HWCFGR0rs> {
        DUAL_W::new(self, 0)
    }
    ///Bits 4:7 - Pseudonoise wave generation capability
    #[inline(always)]
    pub fn lfsr(&mut self) -> LFSR_W<'_, IP_HWCFGR0rs> {
        LFSR_W::new(self, 4)
    }
    ///Bits 8:11 - Triangle wave generation capability
    #[inline(always)]
    pub fn triangle(&mut self) -> TRIANGLE_W<'_, IP_HWCFGR0rs> {
        TRIANGLE_W::new(self, 8)
    }
    ///Bits 12:15 - Sample and hold mode capability
    #[inline(always)]
    pub fn sample(&mut self) -> SAMPLE_W<'_, IP_HWCFGR0rs> {
        SAMPLE_W::new(self, 12)
    }
    ///Bits 16:23 - option register bit width
    #[inline(always)]
    pub fn or_cfg(&mut self) -> OR_CFG_W<'_, IP_HWCFGR0rs> {
        OR_CFG_W::new(self, 16)
    }
}
/**DAC IP Hardware Configuration Register

You can [`read`](crate::Reg::read) this register and get [`ip_hwcfgr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ip_hwcfgr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G081.html#DAC:IP_HWCFGR0)*/
pub struct IP_HWCFGR0rs;
impl crate::RegisterSpec for IP_HWCFGR0rs {
    type Ux = u32;
}
///`read()` method returns [`ip_hwcfgr0::R`](R) reader structure
impl crate::Readable for IP_HWCFGR0rs {}
///`write(|w| ..)` method takes [`ip_hwcfgr0::W`](W) writer structure
impl crate::Writable for IP_HWCFGR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IP_HWCFGR0 to value 0x1111
impl crate::Resettable for IP_HWCFGR0rs {
    const RESET_VALUE: u32 = 0x1111;
}
