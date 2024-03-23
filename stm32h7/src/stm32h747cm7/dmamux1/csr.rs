#[doc = "Register `CSR` reader"]
pub type R = crate::R<CSRrs>;
#[doc = "Field `SOF(0-15)` reader - Synchronization overrun event flag"]
pub type SOF_R = crate::BitReader;
impl R {
    #[doc = "Synchronization overrun event flag"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SOF0` field"]
    #[inline(always)]
    pub fn sof(&self, n: u8) -> SOF_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        SOF_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof_iter(&self) -> impl Iterator<Item = SOF_R> + '_ {
        (0..16).map(move |n| SOF_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof0(&self) -> SOF_R {
        SOF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof1(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof2(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof3(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof4(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof5(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof6(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof7(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof8(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof9(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof10(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof11(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof12(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof13(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof14(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof15(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "DMAMUX request line multiplexer interrupt channel status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CSRrs {}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSRrs {
    const RESET_VALUE: u32 = 0;
}
