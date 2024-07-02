///Register `FPSCR` reader
pub type R = crate::R<FPSCRrs>;
///Register `FPSCR` writer
pub type W = crate::W<FPSCRrs>;
///Field `IOC` reader - Invalid operation cumulative exception bit
pub type IOC_R = crate::BitReader;
///Field `IOC` writer - Invalid operation cumulative exception bit
pub type IOC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DZC` reader - Division by zero cumulative exception bit.
pub type DZC_R = crate::BitReader;
///Field `DZC` writer - Division by zero cumulative exception bit.
pub type DZC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OFC` reader - Overflow cumulative exception bit
pub type OFC_R = crate::BitReader;
///Field `OFC` writer - Overflow cumulative exception bit
pub type OFC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UFC` reader - Underflow cumulative exception bit
pub type UFC_R = crate::BitReader;
///Field `UFC` writer - Underflow cumulative exception bit
pub type UFC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IXC` reader - Inexact cumulative exception bit
pub type IXC_R = crate::BitReader;
///Field `IXC` writer - Inexact cumulative exception bit
pub type IXC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IDC` reader - Input denormal cumulative exception bit.
pub type IDC_R = crate::BitReader;
///Field `IDC` writer - Input denormal cumulative exception bit.
pub type IDC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RMode` reader - Rounding Mode control field
pub type RMODE_R = crate::FieldReader;
///Field `RMode` writer - Rounding Mode control field
pub type RMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `FZ` reader - Flush-to-zero mode control bit:
pub type FZ_R = crate::BitReader;
///Field `FZ` writer - Flush-to-zero mode control bit:
pub type FZ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DN` reader - Default NaN mode control bit
pub type DN_R = crate::BitReader;
///Field `DN` writer - Default NaN mode control bit
pub type DN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHP` reader - Alternative half-precision control bit
pub type AHP_R = crate::BitReader;
///Field `AHP` writer - Alternative half-precision control bit
pub type AHP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `V` reader - Overflow condition code flag
pub type V_R = crate::BitReader;
///Field `V` writer - Overflow condition code flag
pub type V_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `C` reader - Carry condition code flag
pub type C_R = crate::BitReader;
///Field `C` writer - Carry condition code flag
pub type C_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `Z` reader - Zero condition code flag
pub type Z_R = crate::BitReader;
///Field `Z` writer - Zero condition code flag
pub type Z_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `N` reader - Negative condition code flag
pub type N_R = crate::BitReader;
///Field `N` writer - Negative condition code flag
pub type N_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Invalid operation cumulative exception bit
    #[inline(always)]
    pub fn ioc(&self) -> IOC_R {
        IOC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Division by zero cumulative exception bit.
    #[inline(always)]
    pub fn dzc(&self) -> DZC_R {
        DZC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Overflow cumulative exception bit
    #[inline(always)]
    pub fn ofc(&self) -> OFC_R {
        OFC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Underflow cumulative exception bit
    #[inline(always)]
    pub fn ufc(&self) -> UFC_R {
        UFC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Inexact cumulative exception bit
    #[inline(always)]
    pub fn ixc(&self) -> IXC_R {
        IXC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - Input denormal cumulative exception bit.
    #[inline(always)]
    pub fn idc(&self) -> IDC_R {
        IDC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 22:23 - Rounding Mode control field
    #[inline(always)]
    pub fn rmode(&self) -> RMODE_R {
        RMODE_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bit 24 - Flush-to-zero mode control bit:
    #[inline(always)]
    pub fn fz(&self) -> FZ_R {
        FZ_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Default NaN mode control bit
    #[inline(always)]
    pub fn dn(&self) -> DN_R {
        DN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Alternative half-precision control bit
    #[inline(always)]
    pub fn ahp(&self) -> AHP_R {
        AHP_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 28 - Overflow condition code flag
    #[inline(always)]
    pub fn v(&self) -> V_R {
        V_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Carry condition code flag
    #[inline(always)]
    pub fn c(&self) -> C_R {
        C_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Zero condition code flag
    #[inline(always)]
    pub fn z(&self) -> Z_R {
        Z_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Negative condition code flag
    #[inline(always)]
    pub fn n(&self) -> N_R {
        N_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPSCR")
            .field("ioc", &self.ioc())
            .field("dzc", &self.dzc())
            .field("ofc", &self.ofc())
            .field("ufc", &self.ufc())
            .field("ixc", &self.ixc())
            .field("idc", &self.idc())
            .field("rmode", &self.rmode())
            .field("fz", &self.fz())
            .field("dn", &self.dn())
            .field("ahp", &self.ahp())
            .field("v", &self.v())
            .field("c", &self.c())
            .field("z", &self.z())
            .field("n", &self.n())
            .finish()
    }
}
impl W {
    ///Bit 0 - Invalid operation cumulative exception bit
    #[inline(always)]
    #[must_use]
    pub fn ioc(&mut self) -> IOC_W<FPSCRrs> {
        IOC_W::new(self, 0)
    }
    ///Bit 1 - Division by zero cumulative exception bit.
    #[inline(always)]
    #[must_use]
    pub fn dzc(&mut self) -> DZC_W<FPSCRrs> {
        DZC_W::new(self, 1)
    }
    ///Bit 2 - Overflow cumulative exception bit
    #[inline(always)]
    #[must_use]
    pub fn ofc(&mut self) -> OFC_W<FPSCRrs> {
        OFC_W::new(self, 2)
    }
    ///Bit 3 - Underflow cumulative exception bit
    #[inline(always)]
    #[must_use]
    pub fn ufc(&mut self) -> UFC_W<FPSCRrs> {
        UFC_W::new(self, 3)
    }
    ///Bit 4 - Inexact cumulative exception bit
    #[inline(always)]
    #[must_use]
    pub fn ixc(&mut self) -> IXC_W<FPSCRrs> {
        IXC_W::new(self, 4)
    }
    ///Bit 7 - Input denormal cumulative exception bit.
    #[inline(always)]
    #[must_use]
    pub fn idc(&mut self) -> IDC_W<FPSCRrs> {
        IDC_W::new(self, 7)
    }
    ///Bits 22:23 - Rounding Mode control field
    #[inline(always)]
    #[must_use]
    pub fn rmode(&mut self) -> RMODE_W<FPSCRrs> {
        RMODE_W::new(self, 22)
    }
    ///Bit 24 - Flush-to-zero mode control bit:
    #[inline(always)]
    #[must_use]
    pub fn fz(&mut self) -> FZ_W<FPSCRrs> {
        FZ_W::new(self, 24)
    }
    ///Bit 25 - Default NaN mode control bit
    #[inline(always)]
    #[must_use]
    pub fn dn(&mut self) -> DN_W<FPSCRrs> {
        DN_W::new(self, 25)
    }
    ///Bit 26 - Alternative half-precision control bit
    #[inline(always)]
    #[must_use]
    pub fn ahp(&mut self) -> AHP_W<FPSCRrs> {
        AHP_W::new(self, 26)
    }
    ///Bit 28 - Overflow condition code flag
    #[inline(always)]
    #[must_use]
    pub fn v(&mut self) -> V_W<FPSCRrs> {
        V_W::new(self, 28)
    }
    ///Bit 29 - Carry condition code flag
    #[inline(always)]
    #[must_use]
    pub fn c(&mut self) -> C_W<FPSCRrs> {
        C_W::new(self, 29)
    }
    ///Bit 30 - Zero condition code flag
    #[inline(always)]
    #[must_use]
    pub fn z(&mut self) -> Z_W<FPSCRrs> {
        Z_W::new(self, 30)
    }
    ///Bit 31 - Negative condition code flag
    #[inline(always)]
    #[must_use]
    pub fn n(&mut self) -> N_W<FPSCRrs> {
        N_W::new(self, 31)
    }
}
/**Floating-point status control register

You can [`read`](crate::Reg::read) this register and get [`fpscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G041.html#FPU:FPSCR)*/
pub struct FPSCRrs;
impl crate::RegisterSpec for FPSCRrs {
    type Ux = u32;
}
///`read()` method returns [`fpscr::R`](R) reader structure
impl crate::Readable for FPSCRrs {}
///`write(|w| ..)` method takes [`fpscr::W`](W) writer structure
impl crate::Writable for FPSCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FPSCR to value 0
impl crate::Resettable for FPSCRrs {
    const RESET_VALUE: u32 = 0;
}
