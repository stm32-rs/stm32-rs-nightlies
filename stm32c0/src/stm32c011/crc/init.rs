///Register `INIT` reader
pub type R = crate::R<INITrs>;
///Register `INIT` writer
pub type W = crate::W<INITrs>;
///Field `CRC_INIT` reader - Programmable initial CRC value This register is used to write the CRC initial value.
pub type CRC_INIT_R = crate::FieldReader<u32>;
///Field `CRC_INIT` writer - Programmable initial CRC value This register is used to write the CRC initial value.
pub type CRC_INIT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Programmable initial CRC value This register is used to write the CRC initial value.
    #[inline(always)]
    pub fn crc_init(&self) -> CRC_INIT_R {
        CRC_INIT_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INIT")
            .field("crc_init", &self.crc_init())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Programmable initial CRC value This register is used to write the CRC initial value.
    #[inline(always)]
    pub fn crc_init(&mut self) -> CRC_INIT_W<INITrs> {
        CRC_INIT_W::new(self, 0)
    }
}
/**CRC initial value

You can [`read`](crate::Reg::read) this register and get [`init::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`init::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#CRC:INIT)*/
pub struct INITrs;
impl crate::RegisterSpec for INITrs {
    type Ux = u32;
}
///`read()` method returns [`init::R`](R) reader structure
impl crate::Readable for INITrs {}
///`write(|w| ..)` method takes [`init::W`](W) writer structure
impl crate::Writable for INITrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INIT to value 0xffff_ffff
impl crate::Resettable for INITrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
