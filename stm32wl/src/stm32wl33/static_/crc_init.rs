///Register `CRC_INIT` reader
pub type R = crate::R<CRC_INITrs>;
///Register `CRC_INIT` writer
pub type W = crate::W<CRC_INITrs>;
///Field `CRC_INIT_VAL` reader - CRC intialization value
pub type CRC_INIT_VAL_R = crate::FieldReader<u32>;
///Field `CRC_INIT_VAL` writer - CRC intialization value
pub type CRC_INIT_VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CRC intialization value
    #[inline(always)]
    pub fn crc_init_val(&self) -> CRC_INIT_VAL_R {
        CRC_INIT_VAL_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRC_INIT")
            .field("crc_init_val", &self.crc_init_val())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CRC intialization value
    #[inline(always)]
    pub fn crc_init_val(&mut self) -> CRC_INIT_VAL_W<'_, CRC_INITrs> {
        CRC_INIT_VAL_W::new(self, 0)
    }
}
/**CRC_INIT register

You can [`read`](crate::Reg::read) this register and get [`crc_init::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_init::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATIC:CRC_INIT)*/
pub struct CRC_INITrs;
impl crate::RegisterSpec for CRC_INITrs {
    type Ux = u32;
}
///`read()` method returns [`crc_init::R`](R) reader structure
impl crate::Readable for CRC_INITrs {}
///`write(|w| ..)` method takes [`crc_init::W`](W) writer structure
impl crate::Writable for CRC_INITrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CRC_INIT to value 0
impl crate::Resettable for CRC_INITrs {}
