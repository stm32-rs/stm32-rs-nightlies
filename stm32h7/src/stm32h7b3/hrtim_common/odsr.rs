///Register `ODSR` reader
pub type R = crate::R<ODSRrs>;
///Field `TA1ODS` reader - Timer A Output 1 disable status
pub type TA1ODS_R = crate::BitReader;
///Field `TA2ODS` reader - Timer A Output 2 disable status
pub type TA2ODS_R = crate::BitReader;
///Field `TB1ODS` reader - Timer B Output 1 disable status
pub type TB1ODS_R = crate::BitReader;
///Field `TB2ODS` reader - Timer B Output 2 disable status
pub type TB2ODS_R = crate::BitReader;
///Field `TC1ODS` reader - Timer C Output 1 disable status
pub type TC1ODS_R = crate::BitReader;
///Field `TC2ODS` reader - Timer C Output 2 disable status
pub type TC2ODS_R = crate::BitReader;
///Field `TD1ODS` reader - Timer D Output 1 disable status
pub type TD1ODS_R = crate::BitReader;
///Field `TD2ODS` reader - Timer D Output 2 disable status
pub type TD2ODS_R = crate::BitReader;
///Field `TE1ODS` reader - Timer E Output 1 disable status
pub type TE1ODS_R = crate::BitReader;
///Field `TE2ODS` reader - Timer E Output 2 disable status
pub type TE2ODS_R = crate::BitReader;
impl R {
    ///Bit 0 - Timer A Output 1 disable status
    #[inline(always)]
    pub fn ta1ods(&self) -> TA1ODS_R {
        TA1ODS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Timer A Output 2 disable status
    #[inline(always)]
    pub fn ta2ods(&self) -> TA2ODS_R {
        TA2ODS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Timer B Output 1 disable status
    #[inline(always)]
    pub fn tb1ods(&self) -> TB1ODS_R {
        TB1ODS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timer B Output 2 disable status
    #[inline(always)]
    pub fn tb2ods(&self) -> TB2ODS_R {
        TB2ODS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Timer C Output 1 disable status
    #[inline(always)]
    pub fn tc1ods(&self) -> TC1ODS_R {
        TC1ODS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Timer C Output 2 disable status
    #[inline(always)]
    pub fn tc2ods(&self) -> TC2ODS_R {
        TC2ODS_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Timer D Output 1 disable status
    #[inline(always)]
    pub fn td1ods(&self) -> TD1ODS_R {
        TD1ODS_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Timer D Output 2 disable status
    #[inline(always)]
    pub fn td2ods(&self) -> TD2ODS_R {
        TD2ODS_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Timer E Output 1 disable status
    #[inline(always)]
    pub fn te1ods(&self) -> TE1ODS_R {
        TE1ODS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Timer E Output 2 disable status
    #[inline(always)]
    pub fn te2ods(&self) -> TE2ODS_R {
        TE2ODS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ODSR")
            .field("te2ods", &self.te2ods())
            .field("te1ods", &self.te1ods())
            .field("td2ods", &self.td2ods())
            .field("td1ods", &self.td1ods())
            .field("tc2ods", &self.tc2ods())
            .field("tc1ods", &self.tc1ods())
            .field("tb2ods", &self.tb2ods())
            .field("tb1ods", &self.tb1ods())
            .field("ta2ods", &self.ta2ods())
            .field("ta1ods", &self.ta1ods())
            .finish()
    }
}
/**Output Disable Status Register

You can [`read`](crate::Reg::read) this register and get [`odsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_Common:ODSR)*/
pub struct ODSRrs;
impl crate::RegisterSpec for ODSRrs {
    type Ux = u32;
}
///`read()` method returns [`odsr::R`](R) reader structure
impl crate::Readable for ODSRrs {}
///`reset()` method sets ODSR to value 0
impl crate::Resettable for ODSRrs {
    const RESET_VALUE: u32 = 0;
}
