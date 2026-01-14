///Register `ODSR` reader
pub type R = crate::R<ODSRrs>;
/**Timer %s Output 1 disable status

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TODS {
    ///0: Output disabled in idle state
    Idle = 0,
    ///1: Output disabled in fault state
    Fault = 1,
}
impl From<TODS> for bool {
    #[inline(always)]
    fn from(variant: TODS) -> Self {
        variant as u8 != 0
    }
}
///Field `T1ODS(A,B,C,D,E)` reader - Timer %s Output 1 disable status
pub type T1ODS_R = crate::BitReader<TODS>;
impl T1ODS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TODS {
        match self.bits {
            false => TODS::Idle,
            true => TODS::Fault,
        }
    }
    ///Output disabled in idle state
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == TODS::Idle
    }
    ///Output disabled in fault state
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == TODS::Fault
    }
}
///Field `T2ODS(A,B,C,D,E)` reader - Timer %s Output 2 disable status
pub use T1ODS_R as T2ODS_R;
impl R {
    ///Timer (A,B,C,D,E) Output 1 disable status
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `TA1ODS` field.</div>
    #[inline(always)]
    pub fn t1ods(&self, n: u8) -> T1ODS_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        T1ODS_R::new(((self.bits >> (n * 2)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Timer (A,B,C,D,E) Output 1 disable status
    #[inline(always)]
    pub fn t1ods_iter(&self) -> impl Iterator<Item = T1ODS_R> + '_ {
        (0..5).map(move |n| T1ODS_R::new(((self.bits >> (n * 2)) & 1) != 0))
    }
    ///Bit 0 - Timer A Output 1 disable status
    #[inline(always)]
    pub fn ta1ods(&self) -> T1ODS_R {
        T1ODS_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Timer B Output 1 disable status
    #[inline(always)]
    pub fn tb1ods(&self) -> T1ODS_R {
        T1ODS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - Timer C Output 1 disable status
    #[inline(always)]
    pub fn tc1ods(&self) -> T1ODS_R {
        T1ODS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - Timer D Output 1 disable status
    #[inline(always)]
    pub fn td1ods(&self) -> T1ODS_R {
        T1ODS_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Timer E Output 1 disable status
    #[inline(always)]
    pub fn te1ods(&self) -> T1ODS_R {
        T1ODS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Timer (A,B,C,D,E) Output 2 disable status
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `TA2ODS` field.</div>
    #[inline(always)]
    pub fn t2ods(&self, n: u8) -> T2ODS_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        T2ODS_R::new(((self.bits >> (n * 2 + 1)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Timer (A,B,C,D,E) Output 2 disable status
    #[inline(always)]
    pub fn t2ods_iter(&self) -> impl Iterator<Item = T2ODS_R> + '_ {
        (0..5).map(move |n| T2ODS_R::new(((self.bits >> (n * 2 + 1)) & 1) != 0))
    }
    ///Bit 1 - Timer A Output 2 disable status
    #[inline(always)]
    pub fn ta2ods(&self) -> T2ODS_R {
        T2ODS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Timer B Output 2 disable status
    #[inline(always)]
    pub fn tb2ods(&self) -> T2ODS_R {
        T2ODS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - Timer C Output 2 disable status
    #[inline(always)]
    pub fn tc2ods(&self) -> T2ODS_R {
        T2ODS_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - Timer D Output 2 disable status
    #[inline(always)]
    pub fn td2ods(&self) -> T2ODS_R {
        T2ODS_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - Timer E Output 2 disable status
    #[inline(always)]
    pub fn te2ods(&self) -> T2ODS_R {
        T2ODS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ODSR")
            .field("ta1ods", &self.ta1ods())
            .field("tb1ods", &self.tb1ods())
            .field("tc1ods", &self.tc1ods())
            .field("td1ods", &self.td1ods())
            .field("te1ods", &self.te1ods())
            .field("ta2ods", &self.ta2ods())
            .field("tb2ods", &self.tb2ods())
            .field("tc2ods", &self.tc2ods())
            .field("td2ods", &self.td2ods())
            .field("te2ods", &self.te2ods())
            .finish()
    }
}
/**Output Disable Status Register

You can [`read`](crate::Reg::read) this register and get [`odsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM7.html#HRTIM_Common:ODSR)*/
pub struct ODSRrs;
impl crate::RegisterSpec for ODSRrs {
    type Ux = u32;
}
///`read()` method returns [`odsr::R`](R) reader structure
impl crate::Readable for ODSRrs {}
///`reset()` method sets ODSR to value 0
impl crate::Resettable for ODSRrs {}
