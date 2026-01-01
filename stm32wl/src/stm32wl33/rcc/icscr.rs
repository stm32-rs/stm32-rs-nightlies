///Register `ICSCR` reader
pub type R = crate::R<ICSCRrs>;
///Register `ICSCR` writer
pub type W = crate::W<ICSCRrs>;
///Field `LSITRIMEN` reader - Low Speed oscillator trimming enable Set and reset by software. Reset source only for this field: PORESETn 0: LSI oscillator Bias trimming disabled 1: LSI oscillator Bias trimming enabled
pub type LSITRIMEN_R = crate::BitReader;
///Field `LSITRIMEN` writer - Low Speed oscillator trimming enable Set and reset by software. Reset source only for this field: PORESETn 0: LSI oscillator Bias trimming disabled 1: LSI oscillator Bias trimming enabled
pub type LSITRIMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSITRIMOK` reader - LSITRIMOK: Low Speed oscillator trimming OK Set and reset by hardware to indicate when the Low Speed Internal RC oscillator has reached an optimal trimming of its bias current; this bit is only valid when LSITRIMEN is active. 0: LSI Bias trimming (LSIBW) is not good 1: LSI Bias trimming (LSIBW) value is OK
pub type LSITRIMOK_R = crate::BitReader;
///Field `LSIBW` reader - Trimming in test mode The value stored is the correspondent Engi Byte and represents the actual value driving the input of the hardware macro. This value is loaded soon after the completion of the Option Byte Loading procedure. This field is directly writeable only in Test Mode.
pub type LSIBW_R = crate::FieldReader;
///Field `HSITRIMOFFSET` reader - ICSCR\[18:16\] = HSITRIMOFFSET\[2:0\]: High Speed oscillator signed trimming offset 000: 0 (+ 0 MHz / default) 001: 1 (-0.5 MHz) 010: 2 (-1MHz) 011: 3 (-1.5 MHz) 100: -1 (+2 MHz) 101: -2 (+1.5MHz) 110: -3 (+1 MHz) 111: -4 (+0.5 MHz)
pub type HSITRIMOFFSET_R = crate::FieldReader;
///Field `HSITRIMOFFSET` writer - ICSCR\[18:16\] = HSITRIMOFFSET\[2:0\]: High Speed oscillator signed trimming offset 000: 0 (+ 0 MHz / default) 001: 1 (-0.5 MHz) 010: 2 (-1MHz) 011: 3 (-1.5 MHz) 100: -1 (+2 MHz) 101: -2 (+1.5MHz) 110: -3 (+1 MHz) 111: -4 (+0.5 MHz)
pub type HSITRIMOFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `HSITRIM` reader - High Speed Internal clock trimming. This value is loaded soon after the completion of the Option Byte Loading procedure. When max value 0x3f is set, HSI is less than 64MHz
pub type HSITRIM_R = crate::FieldReader;
impl R {
    ///Bit 0 - Low Speed oscillator trimming enable Set and reset by software. Reset source only for this field: PORESETn 0: LSI oscillator Bias trimming disabled 1: LSI oscillator Bias trimming enabled
    #[inline(always)]
    pub fn lsitrimen(&self) -> LSITRIMEN_R {
        LSITRIMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSITRIMOK: Low Speed oscillator trimming OK Set and reset by hardware to indicate when the Low Speed Internal RC oscillator has reached an optimal trimming of its bias current; this bit is only valid when LSITRIMEN is active. 0: LSI Bias trimming (LSIBW) is not good 1: LSI Bias trimming (LSIBW) value is OK
    #[inline(always)]
    pub fn lsitrimok(&self) -> LSITRIMOK_R {
        LSITRIMOK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:5 - Trimming in test mode The value stored is the correspondent Engi Byte and represents the actual value driving the input of the hardware macro. This value is loaded soon after the completion of the Option Byte Loading procedure. This field is directly writeable only in Test Mode.
    #[inline(always)]
    pub fn lsibw(&self) -> LSIBW_R {
        LSIBW_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    ///Bits 16:18 - ICSCR\[18:16\] = HSITRIMOFFSET\[2:0\]: High Speed oscillator signed trimming offset 000: 0 (+ 0 MHz / default) 001: 1 (-0.5 MHz) 010: 2 (-1MHz) 011: 3 (-1.5 MHz) 100: -1 (+2 MHz) 101: -2 (+1.5MHz) 110: -3 (+1 MHz) 111: -4 (+0.5 MHz)
    #[inline(always)]
    pub fn hsitrimoffset(&self) -> HSITRIMOFFSET_R {
        HSITRIMOFFSET_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 24:29 - High Speed Internal clock trimming. This value is loaded soon after the completion of the Option Byte Loading procedure. When max value 0x3f is set, HSI is less than 64MHz
    #[inline(always)]
    pub fn hsitrim(&self) -> HSITRIM_R {
        HSITRIM_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICSCR")
            .field("lsitrimen", &self.lsitrimen())
            .field("lsitrimok", &self.lsitrimok())
            .field("lsibw", &self.lsibw())
            .field("hsitrimoffset", &self.hsitrimoffset())
            .field("hsitrim", &self.hsitrim())
            .finish()
    }
}
impl W {
    ///Bit 0 - Low Speed oscillator trimming enable Set and reset by software. Reset source only for this field: PORESETn 0: LSI oscillator Bias trimming disabled 1: LSI oscillator Bias trimming enabled
    #[inline(always)]
    pub fn lsitrimen(&mut self) -> LSITRIMEN_W<'_, ICSCRrs> {
        LSITRIMEN_W::new(self, 0)
    }
    ///Bits 16:18 - ICSCR\[18:16\] = HSITRIMOFFSET\[2:0\]: High Speed oscillator signed trimming offset 000: 0 (+ 0 MHz / default) 001: 1 (-0.5 MHz) 010: 2 (-1MHz) 011: 3 (-1.5 MHz) 100: -1 (+2 MHz) 101: -2 (+1.5MHz) 110: -3 (+1 MHz) 111: -4 (+0.5 MHz)
    #[inline(always)]
    pub fn hsitrimoffset(&mut self) -> HSITRIMOFFSET_W<'_, ICSCRrs> {
        HSITRIMOFFSET_W::new(self, 16)
    }
}
/**ICSCR register

You can [`read`](crate::Reg::read) this register and get [`icscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RCC:ICSCR)*/
pub struct ICSCRrs;
impl crate::RegisterSpec for ICSCRrs {
    type Ux = u32;
}
///`read()` method returns [`icscr::R`](R) reader structure
impl crate::Readable for ICSCRrs {}
///`write(|w| ..)` method takes [`icscr::W`](W) writer structure
impl crate::Writable for ICSCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICSCR to value 0x3f00_0000
impl crate::Resettable for ICSCRrs {
    const RESET_VALUE: u32 = 0x3f00_0000;
}
