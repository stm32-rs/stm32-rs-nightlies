///Register `HWCFGR0` reader
pub type R = crate::R<HWCFGR0rs>;
///Field `OR_RES` reader - Option register reset value
pub type OR_RES_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Option register reset value
    #[inline(always)]
    pub fn or_res(&self) -> OR_RES_R {
        OR_RES_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR0")
            .field("or_res", &self.or_res())
            .finish()
    }
}
/**GPIO port N hardware configuration register 0

You can [`read`](crate::Reg::read) this register and get [`hwcfgr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#GPION:HWCFGR0)*/
pub struct HWCFGR0rs;
impl crate::RegisterSpec for HWCFGR0rs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr0::R`](R) reader structure
impl crate::Readable for HWCFGR0rs {}
///`reset()` method sets HWCFGR0 to value 0
impl crate::Resettable for HWCFGR0rs {}
