#[doc = "Register `HDPLSR` reader"]
pub type R = crate::R<HDPLSRrs>;
#[doc = "Field `HDPL` reader - temporal isolation level This bitfield returns the current temporal isolation level."]
pub type HDPL_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - temporal isolation level This bitfield returns the current temporal isolation level."]
    #[inline(always)]
    pub fn hdpl(&self) -> HDPL_R {
        HDPL_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "SBS temporal isolation status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdplsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HDPLSRrs;
impl crate::RegisterSpec for HDPLSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hdplsr::R`](R) reader structure"]
impl crate::Readable for HDPLSRrs {}
#[doc = "`reset()` method sets HDPLSR to value 0"]
impl crate::Resettable for HDPLSRrs {
    const RESET_VALUE: u32 = 0;
}
