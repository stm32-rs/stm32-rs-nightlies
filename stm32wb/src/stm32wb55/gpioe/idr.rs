///Register `IDR` reader
pub type R = crate::R<IDRrs>;
/**Port input data pin %s

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDR0 {
    ///0: Input is logic low
    Low = 0,
    ///1: Input is logic high
    High = 1,
}
impl From<IDR0> for bool {
    #[inline(always)]
    fn from(variant: IDR0) -> Self {
        variant as u8 != 0
    }
}
///Field `IDR(0-4)` reader - Port input data pin %s
pub type IDR_R = crate::BitReader<IDR0>;
impl IDR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IDR0 {
        match self.bits {
            false => IDR0::Low,
            true => IDR0::High,
        }
    }
    ///Input is logic low
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == IDR0::Low
    }
    ///Input is logic high
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == IDR0::High
    }
}
impl R {
    ///Port input data pin (0-4)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `IDR0` field.</div>
    #[inline(always)]
    pub fn idr(&self, n: u8) -> IDR_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        IDR_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Port input data pin (0-4)
    #[inline(always)]
    pub fn idr_iter(&self) -> impl Iterator<Item = IDR_R> + '_ {
        (0..5).map(move |n| IDR_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Port input data pin 0
    #[inline(always)]
    pub fn idr0(&self) -> IDR_R {
        IDR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port input data pin 1
    #[inline(always)]
    pub fn idr1(&self) -> IDR_R {
        IDR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port input data pin 2
    #[inline(always)]
    pub fn idr2(&self) -> IDR_R {
        IDR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port input data pin 3
    #[inline(always)]
    pub fn idr3(&self) -> IDR_R {
        IDR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port input data pin 4
    #[inline(always)]
    pub fn idr4(&self) -> IDR_R {
        IDR_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDR")
            .field("idr0", &self.idr0())
            .field("idr1", &self.idr1())
            .field("idr2", &self.idr2())
            .field("idr3", &self.idr3())
            .field("idr4", &self.idr4())
            .finish()
    }
}
/**GPIO port input data register

You can [`read`](crate::Reg::read) this register and get [`idr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#GPIOE:IDR)*/
pub struct IDRrs;
impl crate::RegisterSpec for IDRrs {
    type Ux = u32;
}
///`read()` method returns [`idr::R`](R) reader structure
impl crate::Readable for IDRrs {}
///`reset()` method sets IDR to value 0
impl crate::Resettable for IDRrs {
    const RESET_VALUE: u32 = 0;
}
