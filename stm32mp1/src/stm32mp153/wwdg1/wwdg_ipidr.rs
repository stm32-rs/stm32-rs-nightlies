///Register `WWDG_IPIDR` reader
pub type R = crate::R<WWDG_IPIDRrs>;
///Field `ID` reader - ID
pub type ID_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - ID
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WWDG_IPIDR")
            .field("id", &self.id())
            .finish()
    }
}
/**WWDG ID register

You can [`read`](crate::Reg::read) this register and get [`wwdg_ipidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#WWDG1:WWDG_IPIDR)*/
pub struct WWDG_IPIDRrs;
impl crate::RegisterSpec for WWDG_IPIDRrs {
    type Ux = u32;
}
///`read()` method returns [`wwdg_ipidr::R`](R) reader structure
impl crate::Readable for WWDG_IPIDRrs {}
///`reset()` method sets WWDG_IPIDR to value 0x0012_0051
impl crate::Resettable for WWDG_IPIDRrs {
    const RESET_VALUE: u32 = 0x0012_0051;
}
