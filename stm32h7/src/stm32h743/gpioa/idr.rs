///Register `IDR` reader
pub type R = crate::R<IDRrs>;
/**Port input data pin %s

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INPUT_DATA {
    ///0: Input is logic low
    Low = 0,
    ///1: Input is logic high
    High = 1,
}
impl From<INPUT_DATA> for bool {
    #[inline(always)]
    fn from(variant: INPUT_DATA) -> Self {
        variant as u8 != 0
    }
}
///Field `IDR(0-15)` reader - Port input data pin %s
pub type IDR_R = crate::BitReader<INPUT_DATA>;
impl IDR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INPUT_DATA {
        match self.bits {
            false => INPUT_DATA::Low,
            true => INPUT_DATA::High,
        }
    }
    ///Input is logic low
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == INPUT_DATA::Low
    }
    ///Input is logic high
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == INPUT_DATA::High
    }
}
impl R {
    ///Port input data pin (0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `IDR0` field.</div>
    #[inline(always)]
    pub fn idr(&self, n: u8) -> IDR_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        IDR_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Port input data pin (0-15)
    #[inline(always)]
    pub fn idr_iter(&self) -> impl Iterator<Item = IDR_R> + '_ {
        (0..16).map(move |n| IDR_R::new(((self.bits >> n) & 1) != 0))
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
    ///Bit 5 - Port input data pin 5
    #[inline(always)]
    pub fn idr5(&self) -> IDR_R {
        IDR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Port input data pin 6
    #[inline(always)]
    pub fn idr6(&self) -> IDR_R {
        IDR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Port input data pin 7
    #[inline(always)]
    pub fn idr7(&self) -> IDR_R {
        IDR_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Port input data pin 8
    #[inline(always)]
    pub fn idr8(&self) -> IDR_R {
        IDR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Port input data pin 9
    #[inline(always)]
    pub fn idr9(&self) -> IDR_R {
        IDR_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Port input data pin 10
    #[inline(always)]
    pub fn idr10(&self) -> IDR_R {
        IDR_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Port input data pin 11
    #[inline(always)]
    pub fn idr11(&self) -> IDR_R {
        IDR_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Port input data pin 12
    #[inline(always)]
    pub fn idr12(&self) -> IDR_R {
        IDR_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Port input data pin 13
    #[inline(always)]
    pub fn idr13(&self) -> IDR_R {
        IDR_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Port input data pin 14
    #[inline(always)]
    pub fn idr14(&self) -> IDR_R {
        IDR_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Port input data pin 15
    #[inline(always)]
    pub fn idr15(&self) -> IDR_R {
        IDR_R::new(((self.bits >> 15) & 1) != 0)
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
            .field("idr5", &self.idr5())
            .field("idr6", &self.idr6())
            .field("idr7", &self.idr7())
            .field("idr8", &self.idr8())
            .field("idr9", &self.idr9())
            .field("idr10", &self.idr10())
            .field("idr11", &self.idr11())
            .field("idr12", &self.idr12())
            .field("idr13", &self.idr13())
            .field("idr14", &self.idr14())
            .field("idr15", &self.idr15())
            .finish()
    }
}
/**GPIO port input data register

You can [`read`](crate::Reg::read) this register and get [`idr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#GPIOA:IDR)*/
pub struct IDRrs;
impl crate::RegisterSpec for IDRrs {
    type Ux = u32;
}
///`read()` method returns [`idr::R`](R) reader structure
impl crate::Readable for IDRrs {}
///`reset()` method sets IDR to value 0
impl crate::Resettable for IDRrs {}
