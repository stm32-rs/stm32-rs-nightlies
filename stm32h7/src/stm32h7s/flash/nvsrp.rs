///Register `NVSRP` reader
pub type R = crate::R<NVSRPrs>;
///Register `NVSRP` writer
pub type W = crate::W<NVSRPrs>;
///Field `NVSTATE` reader - Non-volatile state programming Write to change corresponding bits in FLASH_NVSR register: Actual option byte change from close to open is triggered only after memory clear hardware process is confirmed. When NVSTATE=0xB4 (resp. 0x51) writing any other value than 0x51 (resp. 0xB4) triggers an option byte change error (OPTERRF).
pub type NVSTATE_R = crate::FieldReader;
///Field `NVSTATE` writer - Non-volatile state programming Write to change corresponding bits in FLASH_NVSR register: Actual option byte change from close to open is triggered only after memory clear hardware process is confirmed. When NVSTATE=0xB4 (resp. 0x51) writing any other value than 0x51 (resp. 0xB4) triggers an option byte change error (OPTERRF).
pub type NVSTATE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Non-volatile state programming Write to change corresponding bits in FLASH_NVSR register: Actual option byte change from close to open is triggered only after memory clear hardware process is confirmed. When NVSTATE=0xB4 (resp. 0x51) writing any other value than 0x51 (resp. 0xB4) triggers an option byte change error (OPTERRF).
    #[inline(always)]
    pub fn nvstate(&self) -> NVSTATE_R {
        NVSTATE_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NVSRP")
            .field("nvstate", &self.nvstate())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Non-volatile state programming Write to change corresponding bits in FLASH_NVSR register: Actual option byte change from close to open is triggered only after memory clear hardware process is confirmed. When NVSTATE=0xB4 (resp. 0x51) writing any other value than 0x51 (resp. 0xB4) triggers an option byte change error (OPTERRF).
    #[inline(always)]
    pub fn nvstate(&mut self) -> NVSTATE_W<'_, NVSRPrs> {
        NVSTATE_W::new(self, 0)
    }
}
/**FLASH security status register programming

You can [`read`](crate::Reg::read) this register and get [`nvsrp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvsrp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#FLASH:NVSRP)*/
pub struct NVSRPrs;
impl crate::RegisterSpec for NVSRPrs {
    type Ux = u32;
}
///`read()` method returns [`nvsrp::R`](R) reader structure
impl crate::Readable for NVSRPrs {}
///`write(|w| ..)` method takes [`nvsrp::W`](W) writer structure
impl crate::Writable for NVSRPrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets NVSRP to value 0
impl crate::Resettable for NVSRPrs {}
