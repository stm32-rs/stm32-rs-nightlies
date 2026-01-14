///Register `HDPLSR` reader
pub type R = crate::R<HDPLSRrs>;
///Field `HDPL` reader - temporal isolation level This bitfield returns the current temporal isolation level.
pub type HDPL_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - temporal isolation level This bitfield returns the current temporal isolation level.
    #[inline(always)]
    pub fn hdpl(&self) -> HDPL_R {
        HDPL_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HDPLSR")
            .field("hdpl", &self.hdpl())
            .finish()
    }
}
/**SBS temporal isolation status register

You can [`read`](crate::Reg::read) this register and get [`hdplsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#SBS:HDPLSR)*/
pub struct HDPLSRrs;
impl crate::RegisterSpec for HDPLSRrs {
    type Ux = u32;
}
///`read()` method returns [`hdplsr::R`](R) reader structure
impl crate::Readable for HDPLSRrs {}
///`reset()` method sets HDPLSR to value 0
impl crate::Resettable for HDPLSRrs {}
