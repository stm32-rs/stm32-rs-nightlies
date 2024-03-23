#[doc = "Register `ISR0` reader"]
pub type R = crate::R<ISR0rs>;
#[doc = "Field `AE(0-15)` reader - AE%s"]
pub type AE_R = crate::BitReader;
#[doc = "Field `PE(0-4)` reader - PE%s"]
pub type PE_R = crate::BitReader;
impl R {
    #[doc = "AE(0-15)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `AE0` field"]
    #[inline(always)]
    pub fn ae(&self, n: u8) -> AE_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        AE_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "AE(0-15)"]
    #[inline(always)]
    pub fn ae_iter(&self) -> impl Iterator<Item = AE_R> + '_ {
        (0..16).map(move |n| AE_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - AE0"]
    #[inline(always)]
    pub fn ae0(&self) -> AE_R {
        AE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AE1"]
    #[inline(always)]
    pub fn ae1(&self) -> AE_R {
        AE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AE2"]
    #[inline(always)]
    pub fn ae2(&self) -> AE_R {
        AE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AE3"]
    #[inline(always)]
    pub fn ae3(&self) -> AE_R {
        AE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AE4"]
    #[inline(always)]
    pub fn ae4(&self) -> AE_R {
        AE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AE5"]
    #[inline(always)]
    pub fn ae5(&self) -> AE_R {
        AE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AE6"]
    #[inline(always)]
    pub fn ae6(&self) -> AE_R {
        AE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AE7"]
    #[inline(always)]
    pub fn ae7(&self) -> AE_R {
        AE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AE8"]
    #[inline(always)]
    pub fn ae8(&self) -> AE_R {
        AE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AE9"]
    #[inline(always)]
    pub fn ae9(&self) -> AE_R {
        AE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AE10"]
    #[inline(always)]
    pub fn ae10(&self) -> AE_R {
        AE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - AE11"]
    #[inline(always)]
    pub fn ae11(&self) -> AE_R {
        AE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - AE12"]
    #[inline(always)]
    pub fn ae12(&self) -> AE_R {
        AE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - AE13"]
    #[inline(always)]
    pub fn ae13(&self) -> AE_R {
        AE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - AE14"]
    #[inline(always)]
    pub fn ae14(&self) -> AE_R {
        AE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AE15"]
    #[inline(always)]
    pub fn ae15(&self) -> AE_R {
        AE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "PE(0-4)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `PE0` field"]
    #[inline(always)]
    pub fn pe(&self, n: u8) -> PE_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        PE_R::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "PE(0-4)"]
    #[inline(always)]
    pub fn pe_iter(&self) -> impl Iterator<Item = PE_R> + '_ {
        (0..5).map(move |n| PE_R::new(((self.bits >> (n + 16)) & 1) != 0))
    }
    #[doc = "Bit 16 - PE0"]
    #[inline(always)]
    pub fn pe0(&self) -> PE_R {
        PE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PE1"]
    #[inline(always)]
    pub fn pe1(&self) -> PE_R {
        PE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PE2"]
    #[inline(always)]
    pub fn pe2(&self) -> PE_R {
        PE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PE3"]
    #[inline(always)]
    pub fn pe3(&self) -> PE_R {
        PE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PE4"]
    #[inline(always)]
    pub fn pe4(&self) -> PE_R {
        PE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[doc = "DSI Host interrupt and status register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISR0rs;
impl crate::RegisterSpec for ISR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr0::R`](R) reader structure"]
impl crate::Readable for ISR0rs {}
#[doc = "`reset()` method sets ISR0 to value 0"]
impl crate::Resettable for ISR0rs {
    const RESET_VALUE: u32 = 0;
}
