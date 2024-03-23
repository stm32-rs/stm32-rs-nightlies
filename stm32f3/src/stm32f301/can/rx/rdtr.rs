#[doc = "Register `RDTR` reader"]
pub type R = crate::R<RDTRrs>;
#[doc = "Field `DLC` reader - DLC"]
pub type DLC_R = crate::FieldReader;
#[doc = "Field `FMI` reader - FMI"]
pub type FMI_R = crate::FieldReader;
#[doc = "Field `TIME` reader - TIME"]
pub type TIME_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - DLC"]
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - FMI"]
    #[inline(always)]
    pub fn fmi(&self) -> FMI_R {
        FMI_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - TIME"]
    #[inline(always)]
    pub fn time(&self) -> TIME_R {
        TIME_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "receive FIFO mailbox data length control and time stamp register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdtr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDTRrs;
impl crate::RegisterSpec for RDTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdtr::R`](R) reader structure"]
impl crate::Readable for RDTRrs {}
#[doc = "`reset()` method sets RDTR to value 0"]
impl crate::Resettable for RDTRrs {
    const RESET_VALUE: u32 = 0;
}
