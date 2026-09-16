///Register `OTPVLDR%s` reader
pub type R = crate::R<OTPVLDRrs>;
///Field `VLDF(0-31)` reader - Valid flag for shadow register %s
pub type VLDF_R = crate::BitReader;
impl R {
    ///Valid flag for shadow register (0-31)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `VLDF0` field.</div>
    #[inline(always)]
    pub fn vldf(&self, n: u8) -> VLDF_R {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        VLDF_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Valid flag for shadow register (0-31)
    #[inline(always)]
    pub fn vldf_iter(&self) -> impl Iterator<Item = VLDF_R> + '_ {
        (0..32).map(move |n| VLDF_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Valid flag for shadow register 0
    #[inline(always)]
    pub fn vldf0(&self) -> VLDF_R {
        VLDF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Valid flag for shadow register 1
    #[inline(always)]
    pub fn vldf1(&self) -> VLDF_R {
        VLDF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Valid flag for shadow register 2
    #[inline(always)]
    pub fn vldf2(&self) -> VLDF_R {
        VLDF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Valid flag for shadow register 3
    #[inline(always)]
    pub fn vldf3(&self) -> VLDF_R {
        VLDF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Valid flag for shadow register 4
    #[inline(always)]
    pub fn vldf4(&self) -> VLDF_R {
        VLDF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Valid flag for shadow register 5
    #[inline(always)]
    pub fn vldf5(&self) -> VLDF_R {
        VLDF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Valid flag for shadow register 6
    #[inline(always)]
    pub fn vldf6(&self) -> VLDF_R {
        VLDF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Valid flag for shadow register 7
    #[inline(always)]
    pub fn vldf7(&self) -> VLDF_R {
        VLDF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Valid flag for shadow register 8
    #[inline(always)]
    pub fn vldf8(&self) -> VLDF_R {
        VLDF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Valid flag for shadow register 9
    #[inline(always)]
    pub fn vldf9(&self) -> VLDF_R {
        VLDF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Valid flag for shadow register 10
    #[inline(always)]
    pub fn vldf10(&self) -> VLDF_R {
        VLDF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Valid flag for shadow register 11
    #[inline(always)]
    pub fn vldf11(&self) -> VLDF_R {
        VLDF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Valid flag for shadow register 12
    #[inline(always)]
    pub fn vldf12(&self) -> VLDF_R {
        VLDF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Valid flag for shadow register 13
    #[inline(always)]
    pub fn vldf13(&self) -> VLDF_R {
        VLDF_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Valid flag for shadow register 14
    #[inline(always)]
    pub fn vldf14(&self) -> VLDF_R {
        VLDF_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Valid flag for shadow register 15
    #[inline(always)]
    pub fn vldf15(&self) -> VLDF_R {
        VLDF_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Valid flag for shadow register 16
    #[inline(always)]
    pub fn vldf16(&self) -> VLDF_R {
        VLDF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Valid flag for shadow register 17
    #[inline(always)]
    pub fn vldf17(&self) -> VLDF_R {
        VLDF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Valid flag for shadow register 18
    #[inline(always)]
    pub fn vldf18(&self) -> VLDF_R {
        VLDF_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Valid flag for shadow register 19
    #[inline(always)]
    pub fn vldf19(&self) -> VLDF_R {
        VLDF_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Valid flag for shadow register 20
    #[inline(always)]
    pub fn vldf20(&self) -> VLDF_R {
        VLDF_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Valid flag for shadow register 21
    #[inline(always)]
    pub fn vldf21(&self) -> VLDF_R {
        VLDF_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Valid flag for shadow register 22
    #[inline(always)]
    pub fn vldf22(&self) -> VLDF_R {
        VLDF_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Valid flag for shadow register 23
    #[inline(always)]
    pub fn vldf23(&self) -> VLDF_R {
        VLDF_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Valid flag for shadow register 24
    #[inline(always)]
    pub fn vldf24(&self) -> VLDF_R {
        VLDF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Valid flag for shadow register 25
    #[inline(always)]
    pub fn vldf25(&self) -> VLDF_R {
        VLDF_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Valid flag for shadow register 26
    #[inline(always)]
    pub fn vldf26(&self) -> VLDF_R {
        VLDF_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Valid flag for shadow register 27
    #[inline(always)]
    pub fn vldf27(&self) -> VLDF_R {
        VLDF_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Valid flag for shadow register 28
    #[inline(always)]
    pub fn vldf28(&self) -> VLDF_R {
        VLDF_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Valid flag for shadow register 29
    #[inline(always)]
    pub fn vldf29(&self) -> VLDF_R {
        VLDF_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Valid flag for shadow register 30
    #[inline(always)]
    pub fn vldf30(&self) -> VLDF_R {
        VLDF_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Valid flag for shadow register 31
    #[inline(always)]
    pub fn vldf31(&self) -> VLDF_R {
        VLDF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTPVLDR")
            .field("vldf0", &self.vldf0())
            .field("vldf1", &self.vldf1())
            .field("vldf2", &self.vldf2())
            .field("vldf3", &self.vldf3())
            .field("vldf4", &self.vldf4())
            .field("vldf5", &self.vldf5())
            .field("vldf6", &self.vldf6())
            .field("vldf7", &self.vldf7())
            .field("vldf8", &self.vldf8())
            .field("vldf9", &self.vldf9())
            .field("vldf10", &self.vldf10())
            .field("vldf11", &self.vldf11())
            .field("vldf12", &self.vldf12())
            .field("vldf13", &self.vldf13())
            .field("vldf14", &self.vldf14())
            .field("vldf15", &self.vldf15())
            .field("vldf16", &self.vldf16())
            .field("vldf17", &self.vldf17())
            .field("vldf18", &self.vldf18())
            .field("vldf19", &self.vldf19())
            .field("vldf20", &self.vldf20())
            .field("vldf21", &self.vldf21())
            .field("vldf22", &self.vldf22())
            .field("vldf23", &self.vldf23())
            .field("vldf24", &self.vldf24())
            .field("vldf25", &self.vldf25())
            .field("vldf26", &self.vldf26())
            .field("vldf27", &self.vldf27())
            .field("vldf28", &self.vldf28())
            .field("vldf29", &self.vldf29())
            .field("vldf30", &self.vldf30())
            .field("vldf31", &self.vldf31())
            .finish()
    }
}
/**BSEC OTP valid register %s

You can [`read`](crate::Reg::read) this register and get [`otpvldr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#BSEC:OTPVLDR[0])*/
pub struct OTPVLDRrs;
impl crate::RegisterSpec for OTPVLDRrs {
    type Ux = u32;
}
///`read()` method returns [`otpvldr::R`](R) reader structure
impl crate::Readable for OTPVLDRrs {}
///`reset()` method sets OTPVLDR%s to value 0
impl crate::Resettable for OTPVLDRrs {}
