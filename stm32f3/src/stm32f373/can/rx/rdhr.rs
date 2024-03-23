#[doc = "Register `RDHR` reader"]
pub type R = crate::R<RDHRrs>;
#[doc = "Field `DATA(4-7)` reader - DATA%s"]
pub type DATA_R = crate::FieldReader;
impl R {
    #[doc = "DATA(4-7)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `DATA4` field"]
    #[inline(always)]
    pub fn data(&self, n: u8) -> DATA_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        DATA_R::new(((self.bits >> (n * 8)) & 0xff) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "DATA(4-7)"]
    #[inline(always)]
    pub fn data_iter(&self) -> impl Iterator<Item = DATA_R> + '_ {
        (0..4).map(move |n| DATA_R::new(((self.bits >> (n * 8)) & 0xff) as u8))
    }
    #[doc = "Bits 0:7 - DATA4"]
    #[inline(always)]
    pub fn data4(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DATA5"]
    #[inline(always)]
    pub fn data5(&self) -> DATA_R {
        DATA_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DATA6"]
    #[inline(always)]
    pub fn data6(&self) -> DATA_R {
        DATA_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DATA7"]
    #[inline(always)]
    pub fn data7(&self) -> DATA_R {
        DATA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "receive FIFO mailbox data high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdhr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDHRrs;
impl crate::RegisterSpec for RDHRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdhr::R`](R) reader structure"]
impl crate::Readable for RDHRrs {}
#[doc = "`reset()` method sets RDHR to value 0"]
impl crate::Resettable for RDHRrs {
    const RESET_VALUE: u32 = 0;
}
