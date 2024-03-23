#[doc = "Register `IDR` reader"]
pub type R = crate::R<IDRrs>;
#[doc = "Port input data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDR0 {
    #[doc = "0: Input is logic low"]
    Low = 0,
    #[doc = "1: Input is logic high"]
    High = 1,
}
impl From<IDR0> for bool {
    #[inline(always)]
    fn from(variant: IDR0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDR(0-15)` reader - Port input data"]
pub type IDR_R = crate::BitReader<IDR0>;
impl IDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IDR0 {
        match self.bits {
            false => IDR0::Low,
            true => IDR0::High,
        }
    }
    #[doc = "Input is logic low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == IDR0::Low
    }
    #[doc = "Input is logic high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == IDR0::High
    }
}
impl R {
    #[doc = "Port input data"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `IDR0` field"]
    #[inline(always)]
    pub fn idr(&self, n: u8) -> IDR_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        IDR_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Port input data"]
    #[inline(always)]
    pub fn idr_iter(&self) -> impl Iterator<Item = IDR_R> + '_ {
        (0..16).map(move |n| IDR_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Port input data"]
    #[inline(always)]
    pub fn idr0(&self) -> IDR_R {
        IDR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port input data"]
    #[inline(always)]
    pub fn idr1(&self) -> IDR_R {
        IDR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port input data"]
    #[inline(always)]
    pub fn idr2(&self) -> IDR_R {
        IDR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port input data"]
    #[inline(always)]
    pub fn idr3(&self) -> IDR_R {
        IDR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port input data"]
    #[inline(always)]
    pub fn idr4(&self) -> IDR_R {
        IDR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port input data"]
    #[inline(always)]
    pub fn idr5(&self) -> IDR_R {
        IDR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port input data"]
    #[inline(always)]
    pub fn idr6(&self) -> IDR_R {
        IDR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port input data"]
    #[inline(always)]
    pub fn idr7(&self) -> IDR_R {
        IDR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port input data"]
    #[inline(always)]
    pub fn idr8(&self) -> IDR_R {
        IDR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port input data"]
    #[inline(always)]
    pub fn idr9(&self) -> IDR_R {
        IDR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port input data"]
    #[inline(always)]
    pub fn idr10(&self) -> IDR_R {
        IDR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port input data"]
    #[inline(always)]
    pub fn idr11(&self) -> IDR_R {
        IDR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port input data"]
    #[inline(always)]
    pub fn idr12(&self) -> IDR_R {
        IDR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port input data"]
    #[inline(always)]
    pub fn idr13(&self) -> IDR_R {
        IDR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port input data"]
    #[inline(always)]
    pub fn idr14(&self) -> IDR_R {
        IDR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port input data"]
    #[inline(always)]
    pub fn idr15(&self) -> IDR_R {
        IDR_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Port input data register (GPIOn_IDR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDRrs;
impl crate::RegisterSpec for IDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idr::R`](R) reader structure"]
impl crate::Readable for IDRrs {}
#[doc = "`reset()` method sets IDR to value 0"]
impl crate::Resettable for IDRrs {
    const RESET_VALUE: u32 = 0;
}
