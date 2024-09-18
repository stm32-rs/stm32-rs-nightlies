///Register `MDF_OEC3CR` reader
pub type R = crate::R<MDF_OEC3CRrs>;
///Register `MDF_OEC3CR` writer
pub type W = crate::W<MDF_OEC3CRrs>;
/**Field `OFFSET` reader - Offset error compensation Set and cleared by software. If the application attempts to write a new offset value while the previous one is not yet applied, this new offset value is ignored. Reading back the OFFSET\[25:0\]
field will inform the application on the current offset value. OFFSET\[25:0\]
represents the value to be subtracted to the signal before going to the SCALE.*/
pub type OFFSET_R = crate::FieldReader<u32>;
/**Field `OFFSET` writer - Offset error compensation Set and cleared by software. If the application attempts to write a new offset value while the previous one is not yet applied, this new offset value is ignored. Reading back the OFFSET\[25:0\]
field will inform the application on the current offset value. OFFSET\[25:0\]
represents the value to be subtracted to the signal before going to the SCALE.*/
pub type OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    /**Bits 0:25 - Offset error compensation Set and cleared by software. If the application attempts to write a new offset value while the previous one is not yet applied, this new offset value is ignored. Reading back the OFFSET\[25:0\]
    field will inform the application on the current offset value. OFFSET\[25:0\]
    represents the value to be subtracted to the signal before going to the SCALE.*/
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new(self.bits & 0x03ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MDF_OEC3CR")
            .field("offset", &self.offset())
            .finish()
    }
}
impl W {
    /**Bits 0:25 - Offset error compensation Set and cleared by software. If the application attempts to write a new offset value while the previous one is not yet applied, this new offset value is ignored. Reading back the OFFSET\[25:0\]
    field will inform the application on the current offset value. OFFSET\[25:0\]
    represents the value to be subtracted to the signal before going to the SCALE.*/
    #[inline(always)]
    #[must_use]
    pub fn offset(&mut self) -> OFFSET_W<MDF_OEC3CRrs> {
        OFFSET_W::new(self, 0)
    }
}
/**This register contains the offset compensation value.

You can [`read`](crate::Reg::read) this register and get [`mdf_oec3cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_oec3cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#MDF1:MDF_OEC3CR)*/
pub struct MDF_OEC3CRrs;
impl crate::RegisterSpec for MDF_OEC3CRrs {
    type Ux = u32;
}
///`read()` method returns [`mdf_oec3cr::R`](R) reader structure
impl crate::Readable for MDF_OEC3CRrs {}
///`write(|w| ..)` method takes [`mdf_oec3cr::W`](W) writer structure
impl crate::Writable for MDF_OEC3CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MDF_OEC3CR to value 0
impl crate::Resettable for MDF_OEC3CRrs {
    const RESET_VALUE: u32 = 0;
}
